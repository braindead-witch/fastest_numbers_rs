// Optimize a given expression (aka the hard part of this program!)

use crate::{
    math::{
        export::OptimizedResult,
        expression::{BinaryOperator, Number, UnaryOperator},
    },
    syllables::{
        Dictionary, Operator,
        counter::{NumberRepresentation, value_to_words},
    },
};
use std::collections::HashMap;

// Makes program output as silly as possible
const SILLY_MODE: bool = true;

pub fn optimize(check_values_up_to: Number, dictionary: &Dictionary) -> OptimizedResult {
    let mut map_of_best_solutions: HashMap<Number, NumberRepresentation> = HashMap::new();

    // Start by filling map with the default values
    for n in 0..check_values_up_to {
        let value = value_to_words(n, dictionary);
        if let Some(v) = value {
            map_of_best_solutions.insert(n, v);
        }
    }

    let unary_operators = dictionary.operators.iter().filter_map(|op| {
        if let Operator::Unary(x) = &op.operator {
            Some((x, op))
        } else {
            None
        }
    });

    let binary_operators = dictionary.operators.iter().filter_map(|op| {
        if let Operator::Binary(x) = &op.operator {
            Some((x, op))
        } else {
            None
        }
    });

    // TODO: remove code duplication
    let mut max_limit = map_of_best_solutions
        .values()
        .map(|v| v.number_of_syllables)
        .max()
        .expect("Couldn't find number with the maximum number of syllables");

    let mut current_limit = 2;

    while current_limit <= max_limit {
        // Unary operators
        for (operator, dictionary_operator) in unary_operators.clone() {
            let candidates = get_candidates_unary(
                &map_of_best_solutions,
                operator.clone(),
                current_limit,
                dictionary,
            );

            for value in candidates {
                if let Ok(new_value) = operator.apply(value) {
                    // This resulted in a value out of our check-range
                    if new_value >= check_values_up_to {
                        continue;
                    }
                    let cost = map_of_best_solutions
                        .get(&value)
                        .unwrap()
                        .number_of_syllables;
                    let new_cost = cost + dictionary_operator.number_of_syllables;
                    if should_update(&map_of_best_solutions, new_value, new_cost) {
                        let new_representation = NumberRepresentation {
                            value: new_value,
                            number_of_syllables: new_cost,
                            representation: format!(
                                "{} {}",
                                map_of_best_solutions.get(&value).unwrap().representation,
                                dictionary_operator.representation
                            ),
                        };
                        map_of_best_solutions.insert(new_value, new_representation);
                    }
                }
            }
        }

        // Binary operators
        // For binary operators: try to combine `value` with every known `u`
        for (operator, dictionary_operator) in binary_operators.clone() {
            let candidates = get_candidates_binary(
                &map_of_best_solutions,
                operator.clone(),
                current_limit - 2,
                current_limit,
                dictionary,
            );

            for (value_left, value_right) in candidates {
                if let Ok(new_value) = operator.apply(value_left, value_right) {
                    // This resulted in a value out of our check-range
                    if new_value >= check_values_up_to {
                        continue;
                    }
                    let cost_left = map_of_best_solutions
                        .get(&value_left)
                        .unwrap()
                        .number_of_syllables;
                    let cost_right = map_of_best_solutions
                        .get(&value_right)
                        .unwrap()
                        .number_of_syllables;
                    let new_cost = cost_left + cost_right + dictionary_operator.number_of_syllables;
                    if should_update(&map_of_best_solutions, new_value, new_cost) {
                        let new_representation = NumberRepresentation {
                            value: new_value,
                            number_of_syllables: new_cost,
                            representation: format!(
                                "{} {} {}",
                                map_of_best_solutions
                                    .get(&value_left)
                                    .unwrap()
                                    .representation,
                                dictionary_operator.representation,
                                map_of_best_solutions
                                    .get(&value_right)
                                    .unwrap()
                                    .representation
                            ),
                        };
                        map_of_best_solutions.insert(new_value, new_representation);
                    }
                }
            }
        }

        current_limit += 1;

        // Update limit, as we might've lowered this limit in a previous iteration
        max_limit = map_of_best_solutions
            .values()
            .map(|v| v.number_of_syllables)
            .max()
            .expect("Couldn't find number with the maximum number of syllables");
    }

    OptimizedResult {
        inner: map_of_best_solutions,
    }
}

fn should_update(
    best_solutions: &HashMap<Number, NumberRepresentation>,
    new_value: Number,
    new_cost: Number,
) -> bool {
    match best_solutions.get(&new_value) {
        Some(existing) => {
            if SILLY_MODE {
                new_cost <= existing.number_of_syllables
            } else {
                new_cost < existing.number_of_syllables
            }
        }
        None => false,
    }
}

fn get_candidates_unary(
    best_solutions: &HashMap<Number, NumberRepresentation>,
    operator: UnaryOperator,
    goal_length: Number,
    dictionary: &Dictionary,
) -> Vec<Number> {
    let operator_length = dictionary
        .get_from_dictionary_operator(Operator::Unary(operator))
        .unwrap()
        .number_of_syllables;
    let mut result = Vec::new();
    for (k, v) in best_solutions {
        // TODO: SILLY_MODE
        if v.number_of_syllables + operator_length < goal_length {
            result.push(*k);
        }
    }
    result
}

fn get_candidates_binary(
    best_solutions: &HashMap<Number, NumberRepresentation>,
    operator: BinaryOperator,
    min_length: Number,
    goal_length: Number,
    dictionary: &Dictionary,
) -> Vec<(Number, Number)> {
    // Try to find all candidates, assuming all numbers of syllable length `min_length` have
    // already been found, meaning we don't have to search for those numbers as solutions anymore.

    // Check syllable length, and prune any combinations that would lead to
    // a syllable length > goal.syllable_length
    let operator_length = dictionary
        .get_from_dictionary_operator(Operator::Binary(operator.clone()))
        .unwrap()
        .number_of_syllables;
    let mut result = Vec::new();

    // Group numbers by syllable length for faster lookup
    let mut by_length: HashMap<Number, Vec<Number>> = HashMap::new();
    for (num, representation) in best_solutions {
        let new_length = representation.number_of_syllables + operator_length;
        if new_length < goal_length {
            by_length
                .entry(representation.number_of_syllables)
                .or_default()
                .push(*num);
        }
    }

    // Find all LHS values, where N_lhs + N_operator < N_goal
    for lhs_length in 1..goal_length - operator_length {
        let Some(lhs_candidates) = by_length.get(&lhs_length) else {
            continue;
        };

        // N_lhs + N_rhs + N_operator < N_goal
        //                      N_rhs < N_goal - N_operator - N_lhs
        let max_rhs_length = goal_length - operator_length - lhs_length;
        if max_rhs_length < 1 {
            continue;
        }

        // If an operator is commutative, we only need to check RHS >= LHS,
        // to avoid duplicates (e.g. 5 + 2 and 2 + 5)
        let min_rhs_length = if operator.is_commutative() {
            lhs_length
        } else {
            1
        };

        for rhs_length in min_rhs_length..max_rhs_length {
            let Some(rhs_candidates) = by_length.get(&rhs_length) else {
                continue;
            };

            let total_length = lhs_length + rhs_length + operator_length;
            if total_length <= min_length || total_length >= goal_length {
                continue;
            }

            for &lhs in lhs_candidates {
                for &rhs in rhs_candidates {
                    // If operator is commutative and the LHS and RHS have the same amount of
                    // syllables, only include LHS <= RHS.
                    if operator.is_commutative() && lhs_length == rhs_length && lhs > rhs {
                        continue;
                    }

                    result.push((lhs, rhs));
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::{
        math::optimize::optimize,
        syllables::{Dictionary, counter::NumberRepresentation},
    };

    #[test]
    fn test_unary() {
        let dictionary = Dictionary::from_file("en-gb.json");
        let optimized = optimize(100, &dictionary);

        assert_eq!(
            optimized.inner.get(&3),
            Some(NumberRepresentation {
                value: 3,
                number_of_syllables: 1,
                representation: "three".to_string()
            })
            .as_ref()
        );
        assert_eq!(
            optimized.inner.get(&81),
            Some(NumberRepresentation {
                value: 81,
                number_of_syllables: 2,
                representation: "nine squared".to_string()
            })
            .as_ref()
        );
        assert_eq!(
            optimized.inner.get(&27),
            Some(NumberRepresentation {
                value: 27,
                number_of_syllables: 2,
                representation: "three cubed".to_string()
            })
            .as_ref()
        );
    }

    #[test]
    fn test_binary() {
        let dictionary = Dictionary::from_file("en-gb.json");
        let optimized = optimize(100, &dictionary);

        assert_eq!(optimized.inner.get(&321).unwrap().number_of_syllables, 6);
    }

    #[test]
    fn test_hard() {
        let dictionary = Dictionary::from_file("en-gb.json");
        let optimized = optimize(100000, &dictionary);

        assert_eq!(optimized.inner.get(&2028).unwrap().number_of_syllables, 5);

        assert_eq!(optimized.inner.get(&1225).unwrap().number_of_syllables, 4);
        assert_eq!(optimized.inner.get(&1234).unwrap().number_of_syllables, 6);
    }
}
