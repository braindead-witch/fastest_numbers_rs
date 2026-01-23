// Optimize a given expression (aka the hard part of this program!)

use crate::{
    math::expression::{BinaryOperator, Number, UnaryOperator},
    syllables::{
        Dictionary, Operator,
        counter::{NumberRepresentation, value_to_words},
    },
};
use std::collections::HashMap;

// Makes program output as silly as possible
const SILLY_MODE: bool = true;

pub fn optimize(
    check_values_up_to: Number,
    dictionary: &Dictionary,
) -> HashMap<Number, NumberRepresentation> {
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

    let mut current_limit = 1;

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
                current_limit - 1,
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

    map_of_best_solutions
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
    // a syllable length >  goal.syllable_length
    let operator_length = dictionary
        .get_from_dictionary_operator(Operator::Binary(operator.clone()))
        .unwrap()
        .number_of_syllables;
    let mut result = Vec::new();

    // First: get all candidates N_first where N_first + N_operator < N_goal.
    // We have found all numbers of length `min_length`, meaning N_first + N_operator can't be
    // smaller or equal too than `min_length`: N_first + N_operator > min_length
    // NOTE: we could include the cases where N_first + N_operator == min_length, which
    // can find alternative solutions (equally optimal). Maybe for SILLY_MODE?
    let mut first_pass_candidates = Vec::new();
    for (k, v) in best_solutions {
        let new_length = v.number_of_syllables + operator_length;
        if new_length < goal_length /* && (new_length >= min_length) */ {
            first_pass_candidates.push(k);
        }
    }

    // Second pass: get all candidates N_second where N_first + N_operator + N_second < N_goal.
    // All values in first_pass_candidates are already N_first < N_goal - N_operator.
    // All values of length `min_length` have been found in a previous iteration, meaning
    // N_first + N_operator + N_second > min_length
    // NOTE: This probably has many mirror duplicates, like (5, 2) and (2, 5). Let's not worry
    // about that for now.
    for lhs in &first_pass_candidates {
        let lhs_length = best_solutions.get(lhs).unwrap().number_of_syllables;
        for rhs in &first_pass_candidates {
            let rhs_length = best_solutions.get(rhs).unwrap().number_of_syllables;
            let new_length = lhs_length + rhs_length + operator_length;
            if (new_length < goal_length) && (new_length > min_length) {
                result.push((**lhs, **rhs));
            }
        }
    }

    result
}

fn get_lhs_candidates(
    operator: &BinaryOperator,
    result: Number,
    max_number_of_syllables: Number,
    max_lhs: Number,
    best_solutions: &HashMap<Number, NumberRepresentation>,
) -> Vec<Number> {
    let mut candidates: Vec<Number> = Vec::new();

    match operator {
        BinaryOperator::Add => {
            // lhs + rhs = result
            for lhs in 1..=result {
                if let Some(lhs_representation) = best_solutions.get(&lhs) {
                    if lhs_representation.number_of_syllables >= max_number_of_syllables {
                        continue;
                    }
                }
                candidates.push(lhs);
            }
        }
        BinaryOperator::Subtract => {
            // lhs - rhs = result
            // => lhs = result + rhs
            // => lhs >= result
            for lhs in result..=max_lhs {
                if let Some(lhs_representation) = best_solutions.get(&lhs) {
                    if lhs_representation.number_of_syllables >= max_number_of_syllables {
                        continue;
                    }
                }
                candidates.push(lhs);
            }
        }
        BinaryOperator::Multiply => {
            // lhs * rhs = result
            // => lhs = result / rhs
            // lhs must be a divisor of result
            for d in divisors(result) {
                if d <= max_lhs {
                    candidates.push(d);
                }
            }
        }
        BinaryOperator::Divide => {
            // lhs / rhs = result
            // => lhs = result * rhs
            // lhs must be a multiple of result
            let mut k = 1;
            while let Some(lhs) = result.checked_mul(k) {
                if lhs > max_lhs {
                    break;
                }
                candidates.push(lhs);
                k += 1;
            }
        }
        BinaryOperator::Exponent => {
            // lhs ^ rhs = result
            // => rhs * log(lhs) = log(result)
            // => log(lhs) = log(result) / rhs
            // => lhs = 10 ^ (log(result) / rhs)

            // TODO: actually prune this better
            for lhs in 1..=max_lhs {
                candidates.push(lhs);
            }
        }
    }

    candidates
}

fn divisors(x: Number) -> Vec<Number> {
    // https://read.learnyard.com/dsa/divisors-of-a-number/
    let mut result = vec![];
    for i in 1..=x.isqrt() {
        if x % i == 0 {
            result.push(i);

            // Check x/i too, and avoid duplicates
            if i != x / i {
                result.push(x / i);
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
            optimized.get(&3),
            Some(NumberRepresentation {
                value: 3,
                number_of_syllables: 1,
                representation: "three".to_string()
            })
            .as_ref()
        );
        assert_eq!(
            optimized.get(&81),
            Some(NumberRepresentation {
                value: 81,
                number_of_syllables: 2,
                representation: "nine squared".to_string()
            })
            .as_ref()
        );
        assert_eq!(
            optimized.get(&27),
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

        // println!("{:#?}", optimized);

        // My algorithm makes this "six plus five", while TheGrayCubers algorithm makes it
        // "ten plus one". Both have the same syllies, so it's ok
        assert_eq!(optimized.get(&11).unwrap().number_of_syllables, 3);
        assert_eq!(optimized.get(&37).unwrap().number_of_syllables, 4);
    }

    #[test]
    fn test_hard() {
        let dictionary = Dictionary::from_file("en-gb.json");
        let optimized = optimize(10000, &dictionary);

        assert_eq!(optimized.get(&1234).unwrap().number_of_syllables, 6);
    }
}
