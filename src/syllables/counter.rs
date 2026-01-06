// https://www.geeksforgeeks.org/dsa/convert-number-to-words/
// Using the recursive algorithm here, with the goal for allowing more general dictionaries, hopefully making language support easier.

use crate::{math::math::Number, syllables::Dictionary};

#[derive(PartialEq, Debug)]
pub struct NumberRepresentation {
    pub representation: String,
    pub number_of_syllables: u32,
}

fn value_to_words_recursive(value: Number, dictionary: &Dictionary) -> NumberRepresentation {
    let mut result = NumberRepresentation { 
        representation: "".to_owned(), 
        number_of_syllables: 0,
    };

    // Iterate over all numeric values
    // Make sure to check in descending order
    let mut sorted_dictionary_constants = dictionary.constants.clone();
    sorted_dictionary_constants.sort_by(|a, b| b.cmp(a));
    for constant in sorted_dictionary_constants {
        // Remove zero: avoids a `n % 0` check.
        if constant.value == 0 {
            continue;
        }

        if value >= constant.value {
            // Language specific rule!
            // TODO: maybe later we can add support for other language number systems, but I'm a Germanic language user myself, so I'll not worry about it right now.
            // TODO: could be implemented with a Language enum + handlers for specific language rules, maybe something like EnglishLanguageRule struct that implements a ValueToWord trait (--> fn value_to_words_recursive(value: Number, dictionary: &Dictionary) -> NumberRepresentation).
            // 400 ---> four hundred
            // 45  -x-> four ten five
            if value >= 100 {
                let new = value_to_words_recursive(value / constant.value, &dictionary);
                result = NumberRepresentation { 
                    representation: result.representation + &new.representation + " ", // Integer division!
                    number_of_syllables: result.number_of_syllables + new.number_of_syllables,
                };
            }

            // Append result for numeric value
            result = NumberRepresentation { 
                representation: result.representation + &constant.representation,
                number_of_syllables: result.number_of_syllables + constant.number_of_syllables,
            };

            // Append remainder
            if constant.value % value > 0 {
                let new = value_to_words_recursive(value % constant.value, &dictionary);
                result = NumberRepresentation { 
                    representation: result.representation + " " + &new.representation, 
                    number_of_syllables: result.number_of_syllables + new.number_of_syllables,
                }
            }

            return result;
        }
    }

    result
}

pub fn value_to_words(value: Number, dictionary: &Dictionary) -> Option<NumberRepresentation> {
    // Special case for zero: in `value_to_words_recursive`, we can't have `n % 0` happening.
    if value == 0 {
        let value = dictionary.get_from_value(0)?;
        return Some(NumberRepresentation { 
            representation: value.representation, 
            number_of_syllables: value.number_of_syllables 
        });
    }
    Some(value_to_words_recursive(value, dictionary))
}
