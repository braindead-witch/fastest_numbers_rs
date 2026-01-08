// Optimize a given expression (aka the hard part of this program!)

// For this, I will be using a Dijkstra graph search using a priority queue.

// This algorithm attempts to find the shortest path between two nodes on a weighted graph.
// In this case, the starting node is the original name and syllable count. This is in the 
// form of 'one hundred twenty-three thousand four hundred fifty-six' for 123456.

// [The algorithm](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Using_a_priority_queue)
// tries to connect a node `v` to a node `u`. 

// The algorithm will take items out of the priority queue, starting from the lowest amount
// of syllables in this case.

use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}};
use crate::{math::math::Number, syllables::{Dictionary, Operator, counter::{NumberRepresentation, value_to_words}}};

// Makes program output as silly as possible
const SILLY_MODE: bool = true;

type PriorityQueueItem = (Reverse<Number>, Number);

pub fn optimize(check_values_up_to: Number, dictionary: &Dictionary) -> HashMap<Number, NumberRepresentation> {
    let mut map_of_best_solutions: HashMap<Number, NumberRepresentation> = HashMap::new();

    // Start by filling map with the default values
    for n in 0..check_values_up_to {
        let value = value_to_words(n, dictionary);
        if let Some(v) = value {
            map_of_best_solutions.insert(n, v);
        }
    }

    // Priority queue. Using Reverse allows us to make a min-heap
    let mut heap: BinaryHeap<PriorityQueueItem> = BinaryHeap::new();
    for (n, best) in &map_of_best_solutions {
        heap.push((Reverse(best.number_of_syllables), *n));
    }

    let operators = dictionary.operators.clone();

    // Loop through queue
    while let Some((Reverse(cost), value)) = heap.pop() {
        // Don't check if this popped value already had its cost changed
        if let Some(current_best) = map_of_best_solutions.get(&value)
            && current_best.number_of_syllables != cost {
                continue;
            }

        // Unary operators; for every unary operator, try to apply to `value`
        // TODO: filter this `operators` to only contain the Unary operators
        for dictionary_operator in &operators {
            if let Operator::Unary(operator) = &dictionary_operator.operator 
                && let Ok(new_value) = operator.apply(value) {
                    // This resulted in a value out of our check-range
                    if new_value >= check_values_up_to {
                        continue;
                    }
                    let new_cost = cost + dictionary_operator.number_of_syllables;
                    if should_update(&map_of_best_solutions, new_value, new_cost) {
                        let new_representation = NumberRepresentation {
                            value: new_value,
                            number_of_syllables: new_cost,
                            representation: format!("{} {}", map_of_best_solutions.get(&value).unwrap().representation, dictionary_operator.representation),
                        };
                        map_of_best_solutions.insert(new_value, new_representation);
                        // Insert our newly-found value back on the priority queue
                        heap.push((Reverse(new_cost), new_value));
                    }
                }
        }

        // For binary operators: try to combine `value` with every known `u`
        // Snapshot current best solutions
        let known_keys: Vec<Number> = map_of_best_solutions.keys().cloned().collect();
        for u in known_keys {
            // TODO: filter this `operators` to only contain the Binary operators
            for dictionary_operator in &operators {
                if let Operator::Binary(operator) = &dictionary_operator.operator 
                    && let Ok(new_value) = operator.apply(value, u) {
                        // This resulted in a value out of our check-range
                        if new_value >= check_values_up_to {
                            continue;
                        }
                        let new_cost = cost + dictionary_operator.number_of_syllables + map_of_best_solutions.get(&u).unwrap().number_of_syllables;
                        if should_update(&map_of_best_solutions, new_value, new_cost) {
                            let new_representation = NumberRepresentation {
                                value: new_value,
                                number_of_syllables: new_cost,
                                representation: format!("{} {} {}", map_of_best_solutions.get(&value).unwrap().representation, dictionary_operator.representation, map_of_best_solutions.get(&u).unwrap().representation),
                            };
                            map_of_best_solutions.insert(new_value, new_representation);
                            // Insert our newly-found value back on the priority queue
                            heap.push((Reverse(new_cost), new_value));
                        }
                    }
            }
        }
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
}
