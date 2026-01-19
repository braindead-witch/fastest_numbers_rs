// Optimize a given expression (aka the hard part of this program!)

// For this, I will be using a Dijkstra graph search using a priority queue.

// This algorithm attempts to find the shortest path between two nodes on a weighted graph.
// In this case, the starting node is the original name and syllable count. This is in the 
// form of 'one hundred twenty-three thousand four hundred fifty-six' for 123456.

// [The algorithm](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Using_a_priority_queue)
// tries to connect a node `v` to a node `u`. 

// The algorithm will take items out of the priority queue, starting from the lowest amount
// of syllables in this case.

use std::collections::HashMap;
use crate::{math::expression::{BinaryOperator, Number, UnaryOperator}, syllables::{Dictionary, Operator, counter::{NumberRepresentation, value_to_words}}};

// Makes program output as silly as possible
const SILLY_MODE: bool = true;

pub fn optimize(check_values_up_to: Number, dictionary: &Dictionary) -> HashMap<Number, NumberRepresentation> {
    let mut map_of_best_solutions: HashMap<Number, NumberRepresentation> = HashMap::new();

    // Start by filling map with the default values
    for n in 0..check_values_up_to {
        let value = value_to_words(n, dictionary);
        if let Some(v) = value {
            map_of_best_solutions.insert(n, v);
        }
    }

    let unary_operators = dictionary.operators
        .iter()
        .filter_map(|op| {
            if let Operator::Unary(x) = &op.operator {
                Some((x, op))
            } else {
                None
            }
        });

    let binary_operators = dictionary.operators
        .iter()
        .filter_map(|op| {
            if let Operator::Binary(x) = &op.operator {
                Some((x, op))
            } else {
                None
            }
        });

    // TODO: remove code duplication
    let mut max_limit = map_of_best_solutions
        .iter()
        .map(|(_k,v)| v.number_of_syllables)
        .max()
        .expect("Couldn't find number with the maximum number of syllables");

    let mut current_limit = 1;

    while current_limit <= max_limit {
        // Unary operators
        for (operator, dictionary_operator) in unary_operators.clone() {
            let candidates = get_candidates_unary(&map_of_best_solutions, operator.clone(), current_limit, dictionary);
    
            for value in candidates {
                if let Ok(new_value) = operator.apply(value) {
                    // This resulted in a value out of our check-range
                    if new_value >= check_values_up_to {
                        continue;
                    }
                    let cost = map_of_best_solutions.get(&value).unwrap().number_of_syllables;
                    let new_cost = cost + dictionary_operator.number_of_syllables;
                    if should_update(&map_of_best_solutions, new_value, new_cost) {
                        let new_representation = NumberRepresentation {
                            value: new_value,
                            number_of_syllables: new_cost,
                            representation: format!("{} {}", map_of_best_solutions.get(&value).unwrap().representation, dictionary_operator.representation),
                        };
                        map_of_best_solutions.insert(new_value, new_representation);
                    }
                }
            }
        }
    
        // Binary operators
        // For binary operators: try to combine `value` with every known `u`
        for (operator, dictionary_operator) in binary_operators.clone() {
            let candidates = get_candidates_binary(&map_of_best_solutions, operator.clone(), current_limit, &dictionary);
    
            for (value_left, value_right) in candidates {
                if let Ok(new_value) = operator.apply(value_left, value_right) {
                    // This resulted in a value out of our check-range
                    if new_value >= check_values_up_to {
                        continue;
                    }
                    let cost_left = map_of_best_solutions.get(&value_left).unwrap().number_of_syllables;
                    let cost_right = map_of_best_solutions.get(&value_right).unwrap().number_of_syllables;
                    let new_cost = cost_left + cost_right + dictionary_operator.number_of_syllables;
                    if should_update(&map_of_best_solutions, new_value, new_cost) {
                        let new_representation = NumberRepresentation {
                            value: new_value,
                            number_of_syllables: new_cost,
                            representation: format!("{} {} {}", map_of_best_solutions.get(&value_left).unwrap().representation, dictionary_operator.representation, map_of_best_solutions.get(&value_right).unwrap().representation),
                        };
                        map_of_best_solutions.insert(new_value, new_representation);
                    }
                }
            }
        }

        current_limit += 1;

        // Update limit, as we might've lowered this limit in a previous iteration
        max_limit = map_of_best_solutions
            .iter()
            .map(|(_k,v)| v.number_of_syllables)
            .max()
            .expect("Couldn't find number with the maximum number of syllables");
    }

    map_of_best_solutions
}

fn should_update(best_solutions: &HashMap<Number, NumberRepresentation>, new_value: Number, new_cost: Number) -> bool {
    match best_solutions.get(&new_value) {
        Some(existing) => if SILLY_MODE {
            new_cost <= existing.number_of_syllables
        } else {
            new_cost < existing.number_of_syllables
        },
        None => false,
    }
}

fn get_candidates_binary(best_solutions: &HashMap<Number, NumberRepresentation>, operator: BinaryOperator, goal_length: Number, dictionary: &Dictionary) -> Vec<(Number, Number)> {
    // Check syllable length, and prune any combinations that would lead to
    // a syllable length >  goal.syllable_length
    let operator_length = dictionary.get_from_dictionary_operator(Operator::Binary(operator)).unwrap().number_of_syllables;
    let mut result = Vec::new();
    
    // First: get all candidates N_first where N_first + N_operator < N_goal
    let mut first_pass_candidates = Vec::new();
    for (k, v) in best_solutions {
        if v.number_of_syllables + operator_length < goal_length {
            first_pass_candidates.push(k);
        }
    }

    // Second pass: get all candidates N_second where N_first + N_operator + N_second < N_goal
    // NOTE: This probably has many mirror duplicates, like (5, 2) and (2, 5). Let's not worry
    // about that for now.
    for (k, v) in best_solutions {
        for u in &first_pass_candidates {
            let first_pass_length = best_solutions.get(u).unwrap().number_of_syllables;
            if v.number_of_syllables + operator_length + first_pass_length < goal_length {
                result.push((**u, *k));
            }
        }
    }

    result
}

fn get_candidates_unary(best_solutions: &HashMap<Number, NumberRepresentation>, operator: UnaryOperator, goal_length: Number, dictionary: &Dictionary) -> Vec<Number> {
    let operator_length = dictionary.get_from_dictionary_operator(Operator::Unary(operator)).unwrap().number_of_syllables;
    let mut result = Vec::new();
    for (k, v) in best_solutions {
        // TODO: SILLY_MODE
        if v.number_of_syllables + operator_length < goal_length {
            result.push(*k);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::{math::optimize::optimize, syllables::{Dictionary, counter::NumberRepresentation}};

    #[test]
    fn test_unary() {
        let dictionary = Dictionary::from_file("en-gb.json");
        let optimized = optimize(100, &dictionary);

        assert_eq!(optimized.get(&3), Some(NumberRepresentation { 
            value: 3,
            number_of_syllables: 1,
            representation: "three".to_string()
        }).as_ref());
        assert_eq!(optimized.get(&81), Some(NumberRepresentation { 
            value: 81,
            number_of_syllables: 2,
            representation: "nine squared".to_string()
        }).as_ref());
        assert_eq!(optimized.get(&27), Some(NumberRepresentation { 
            value: 27,
            number_of_syllables: 2,
            representation: "three cubed".to_string()
        }).as_ref());
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
