// Idea: could make an expression tree for one specific number, using
// Uniform Cost Search; a known algorithm that is guaranteed to find
// the best (read: lowest amount of syllables).
// I could make a function get_next_states that tries every operator,
// and if the resulting state is valid (i.e. result not out of range,
// and PEMDAS is respected), then add those states to the frontier.

use std::{cmp::Reverse, collections::{BinaryHeap, HashMap}};

use crate::{math::expression::{BinaryOperator, Number}, syllables::{Dictionary, Operator, counter::{NumberRepresentation, value_to_words}}};

#[derive(PartialEq, Eq)]
struct PriorityQueueItem {
    cost: Reverse<Number>,
    item: NumberRepresentation,
}

impl PartialOrd for PriorityQueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl Ord for PriorityQueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

fn ucs(start_node: NumberRepresentation, target: Number, dictionary: &Dictionary) -> Option<NumberRepresentation> {
    let mut frontier: BinaryHeap<PriorityQueueItem> = BinaryHeap::new(); // min-heap
    let mut cost_so_far: HashMap<NumberRepresentation, Number> = HashMap::new();

    frontier.push(PriorityQueueItem { cost: Reverse(start_node.number_of_syllables), item: start_node.clone() });
    cost_so_far.insert(start_node.clone(), start_node.number_of_syllables);

    while let Some(current_node) = frontier.pop() {
        // Check if node was already changed in a previous iteration
        if let Some(best_cost) = cost_so_far.get(&current_node.item) {
            if current_node.cost != Reverse(*best_cost) {
                continue;
            }
        }

        if current_node.cost == Reverse(target) {
            return Some(current_node.item);
        }

        for neighbor in get_next_states(current_node.item, dictionary) {
            let neighbor_cost = neighbor.number_of_syllables;
            match cost_so_far.get(&neighbor) {
                Some(&existing_cost) if existing_cost <= neighbor_cost => { },
                _ => {
                    cost_so_far.insert(neighbor.clone(), neighbor_cost);
                    frontier.push(PriorityQueueItem { cost: Reverse(neighbor_cost), item: neighbor });
                }
            }
        }
    }

    None
}

fn get_next_states(start: NumberRepresentation, dictionary: &Dictionary) -> Vec<NumberRepresentation> {
    let mut next_states: Vec<NumberRepresentation> = vec![];

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

    // Unary
    for (operator, dictionary_operator) in unary_operators {
        let inverse = operator.get_inverse(start.value);
        if let Ok(new_value) = inverse {
            let new_representation = value_to_words(new_value, dictionary).expect("Could not convert to word");
            let new_cost = new_representation.number_of_syllables + dictionary_operator.number_of_syllables;
            next_states.push(NumberRepresentation { 
                value: start.value, 
                number_of_syllables: new_cost, 
                representation: format!("{} {}", new_representation.representation, dictionary_operator.representation),
            });
        }
    }

    // Binary
    // let max_lhs = 1000000;
    let max_lhs = start.value * 5;  // TODO: make this better than a random guess
    for (operator, dictionary_operator) in binary_operators {
        let lhs_candidates = get_lhs_candidates(
            operator,
            start.value,
            start.number_of_syllables,
            max_lhs,
            dictionary
        );
        for lhs in lhs_candidates {
            let rhs = operator.get_rhs(start.value, lhs);
            if let Ok(new_value) = rhs && new_value >= 0 {
                let rhs_representation = value_to_words(new_value, dictionary).expect("Could not convert to word");
                let lhs_representation = value_to_words(lhs, dictionary).expect("Could not convert to word");
                let new_cost = rhs_representation.number_of_syllables + dictionary_operator.number_of_syllables + lhs_representation.number_of_syllables;
                if new_cost <= start.number_of_syllables {
                    next_states.push(NumberRepresentation { 
                        value: start.value, 
                        number_of_syllables: new_cost, 
                        representation: format!("{} {} {}", lhs_representation.representation, dictionary_operator.representation, rhs_representation.representation),
                    });
                }
            }
        }
    }

    next_states
}

fn get_lhs_candidates(
    operator: &BinaryOperator,
    result: Number,
    max_number_of_syllables: Number,
    max_lhs: Number,
    dictionary: &Dictionary
) -> Vec<Number> {
    let mut candidates: Vec<Number> = Vec::new();

    match operator {
        BinaryOperator::Add => {
            // lhs + rhs = result
            for lhs in 1..=result {
                if let Some(lhs_representation) = value_to_words(lhs, dictionary) {
                    if lhs_representation.number_of_syllables >= max_number_of_syllables {
                        continue;
                    }
                }
                candidates.push(lhs);
            }
        },
        BinaryOperator::Subtract => {
            // lhs - rhs = result
            // => lhs = result + rhs
            // => lhs >= result
            for lhs in result..=max_lhs {
                if let Some(lhs_representation) = value_to_words(lhs, dictionary) {
                    if lhs_representation.number_of_syllables >= max_number_of_syllables {
                        continue;
                    }
                }
                candidates.push(lhs);
            }
        },
        BinaryOperator::Multiply => {
            // lhs * rhs = result
            // => lhs = result / rhs
            // lhs must be a divisor of result
            for d in divisors(result) {
                if d <= max_lhs {
                    candidates.push(d);
                }
            }
        },
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
        },
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
            if i != x/i {
                result.push(x/i);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::{math::uniform_cost_search::{divisors, ucs}, syllables::{Dictionary, counter::{NumberRepresentation, value_to_words}}};

    #[test]
    fn test_divisors() {
        let mut case1 = divisors(12);
        let mut case2 = divisors(36);
        case1.sort();
        case2.sort();
        assert_eq!(case1, vec![1, 2, 3, 4, 6, 12]);
        assert_eq!(case2, vec![1, 2, 3, 4, 6, 9, 12, 18, 36]);
    }

    #[test]
    fn test_simple() {
        let dictionary = Dictionary::from_file("en-gb.json");
        let result = ucs(
            NumberRepresentation { value: 27, representation: "twenty seven".to_string(), number_of_syllables: 4 },
            2,
            &dictionary
        );
        assert_eq!(result, Some(
            NumberRepresentation { value: 27, representation: "three cubed".to_string(), number_of_syllables: 2 }
        ));
    }

    #[test]
    fn test_medium() {
        let dictionary = Dictionary::from_file("en-gb.json");
        let start = value_to_words(1234, &dictionary).unwrap();
        let result = ucs(
            start,
            6,
            &dictionary
        );
        assert!(result.is_some());
    }
}
