// Optimize a given expression (aka the hard part of this program!)
// As described in the README:

// The process is as follows:
//   - Generate the original name and syllable count for each number in the range. This is in the form of 'one hundred twenty-three thousand four hundred fifty-six' for 123456.
//   - Find all numbers that can be said using 1 syllable.
//   - Find all numbers that can be said using 2 syllables.
//   - Find all numbers that can be said using 3 syllables.
//   - Continue until all numbers in the range have a name.

// During the step of finding names with N syllables, we attempt to construct numbers using results from lower syllable counts. For example:

//   - (N-1 syllables) squared
//   - (N-1 syllables) cubed
//   - (N-2 syllables) plus (1 syllable)
//   - (N-3 syllables) plus (2 syllables)
//   - (N-4 syllables) plus (3 syllables)
//   - and so forth.

use std::collections::HashMap;
use crate::{math::math::Number, syllables::{Dictionary, counter::{NumberRepresentation, value_to_words}}};

pub fn optimize(check_values_up_to: Number, dictionary: &Dictionary) -> HashMap<Number, NumberRepresentation> {
    let mut map_of_best_solutions: HashMap<Number, NumberRepresentation> = HashMap::new();

    // Start by filling map with the default values
    (0..check_values_up_to).for_each(|index| {
        let value = value_to_words(index, dictionary);
        if let Some(v) = value {
            map_of_best_solutions.insert(index, v);
        }
    });

    // Find all numbers that can be said with N syllables
    let max_amount_of_syllables = map_of_best_solutions
        .values()
        .map(|v| v.number_of_syllables)
        .max()
        .unwrap();

    for check_until_syllables in 1..=max_amount_of_syllables {
        // TODO: there's gotta be a way to write this block as a chain of functions
        let mut solutions_under_n_syllables: HashMap<Number, NumberRepresentation> = HashMap::new();
        for (key, value) in &map_of_best_solutions {
            if value.number_of_syllables <= check_until_syllables {
                solutions_under_n_syllables.insert(*key, value.clone());
            }
        }
        
        // For every operator; check if you can replace the left side
        todo!("For every operator; try to build all expression trees that sum up to N, pick the best (lowest-syllable) one");
        // for operator in &dictionary.operators {
        //     match operator.operator {
        //         Operator::Subtract =>
        //     };
        // }
    }

    map_of_best_solutions
}
