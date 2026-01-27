// https://www.geeksforgeeks.org/dsa/convert-number-to-words/
// Using the recursive algorithm here, with the goal for allowing more general dictionaries, hopefully making language support easier.

use serde::{Deserialize, Serialize};

use crate::{language::Dictionary, math::expression::Number};

#[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
pub struct NumberRepresentation {
    pub value: Number,
    pub representation: String,
    pub number_of_syllables: Number,
}

pub struct StringWithSyllableCount {
    pub inner: String,
    pub number_of_syllables: Number,
}

pub trait NumberCompositionRules {
    // Should we include a multiplier word before base?
    // E.g. Dutch: 100 --> honderd (false), 200 --> tweehonderd (true)
    fn needs_multiplier(&self, value: Number, base: Number) -> bool;
    fn multiplier_separator(&self) -> StringWithSyllableCount;
    fn remainder_separator(&self, base: Number) -> StringWithSyllableCount;
    fn remainder_before_base(&self, base: Number) -> bool;
}

pub struct EnglishRules;
impl NumberCompositionRules for EnglishRules {
    fn needs_multiplier(&self, _value: Number, base: Number) -> bool {
        base >= 100
    }

    fn multiplier_separator(&self) -> StringWithSyllableCount {
        StringWithSyllableCount {
            inner: " ".to_owned(),
            number_of_syllables: 0,
        }
    }

    fn remainder_separator(&self, _base: Number) -> StringWithSyllableCount {
        StringWithSyllableCount {
            inner: " ".to_owned(),
            number_of_syllables: 0,
        }
    }

    fn remainder_before_base(&self, _base: Number) -> bool {
        false
    }
}

pub struct DutchRules;
impl NumberCompositionRules for DutchRules {
    // https://onzetaal.nl/taalloket/getallen-uitschrijven
    // "Als je getallen wilt uitschrijven, schrijf je alles zoveel mogelijk aan elkaar. Er komt alleen een spatie na een duizendtal, en de woorden miljoen, miljard, biljoen, enz. schrijf je helemaal los. Deze regel geldt zowel voor hoofdtelwoorden als voor rangtelwoorden. Voor breuken gelden andere regels."
    fn needs_multiplier(&self, value: Number, base: Number) -> bool {
        // No multiplier for 100 (honderd), but we do need one for 200 (tweehonderd)
        let multiplier = value / base;
        base >= 100 && multiplier > 1
    }

    fn multiplier_separator(&self) -> StringWithSyllableCount {
        StringWithSyllableCount {
            inner: "".to_owned(),
            number_of_syllables: 0,
        }
    }

    fn remainder_separator(&self, base: Number) -> StringWithSyllableCount {
        // If the previous word ends in a vowel, "en" should be replaced with "ën".
        // It's definitely possible to hardcode this with
        // `if base.divisible_by(2) || base.divisible_by(3)`, as "twee"
        // and "drie" are the only numbers under 100 that have a vowel at the end.
        // However, hardcoding is not really my thing, so this remains a TODO.
        match base {
            ..100 => StringWithSyllableCount {
                inner: "en".to_owned(),
                number_of_syllables: 1,
            }, // vijfentwintig
            100..1000 => StringWithSyllableCount {
                inner: "".to_owned(),
                number_of_syllables: 0,
            }, // honderdtweeëntwintig
            1000.. => StringWithSyllableCount {
                inner: " ".to_owned(),
                number_of_syllables: 0,
            },
        }
    }

    fn remainder_before_base(&self, base: Number) -> bool {
        (20..100).contains(&base) // "eenentwintig", but not "honderdentwintig"
    }
}

pub enum LanguageRuleset {
    English(EnglishRules),
    Dutch(DutchRules),
    // French, // TODO
}

impl LanguageRuleset {
    fn rules(&self) -> &dyn NumberCompositionRules {
        match self {
            LanguageRuleset::English(english_rules) => english_rules,
            LanguageRuleset::Dutch(dutch_rules) => dutch_rules,
        }
    }

    pub fn value_to_words(
        &self,
        value: Number,
        dictionary: &Dictionary,
    ) -> Option<NumberRepresentation> {
        // Special case for zero: in `value_to_words_recursive`, we can't have `n % 0` happening.
        if value == 0 {
            let value = dictionary.get_from_value(0)?;
            return Some(NumberRepresentation {
                value: value.value,
                representation: value.representation,
                number_of_syllables: value.number_of_syllables,
            });
        }

        // Do not support negative values
        if value < 0 {
            return None;
        }

        self.value_to_words_recursive(value, dictionary)
    }

    fn value_to_words_recursive(
        &self,
        value: Number,
        dictionary: &Dictionary,
    ) -> Option<NumberRepresentation> {
        let rules = self.rules();

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
                let multiplier = value / constant.value;
                let remainder = value % constant.value;

                let mut result = NumberRepresentation {
                    value,
                    representation: String::new(),
                    number_of_syllables: 0,
                };

                // Build up the representation using parts
                let mut parts = vec![];

                // Apply language-specific rules
                if rules.needs_multiplier(value, constant.value) {
                    if let Some(multiplier) = self.value_to_words_recursive(multiplier, dictionary)
                    {
                        parts.push((
                            multiplier.representation.clone(),
                            multiplier.number_of_syllables,
                        ));
                    }

                    let separator = rules.multiplier_separator();
                    parts.push((separator.inner.to_string(), separator.number_of_syllables));
                }

                if remainder > 0 && rules.remainder_before_base(constant.value) {
                    if let Some(remainder) = self.value_to_words_recursive(remainder, dictionary) {
                        parts.push((remainder.representation, remainder.number_of_syllables));
                    }

                    let separator = rules.remainder_separator(constant.value);
                    parts.push((separator.inner.to_string(), separator.number_of_syllables));
                }

                parts.push((constant.representation, constant.number_of_syllables));

                if remainder > 0 && !rules.remainder_before_base(constant.value) {
                    let separator = rules.remainder_separator(constant.value);
                    parts.push((separator.inner.to_string(), separator.number_of_syllables));

                    if let Some(remainder) = self.value_to_words_recursive(remainder, dictionary) {
                        parts.push((remainder.representation, remainder.number_of_syllables));
                    }
                }

                // Combine everything
                for (text, number_of_syllables) in parts {
                    result.representation.push_str(&text);
                    result.number_of_syllables += number_of_syllables;
                }

                return Some(result);
            }
        }

        None
    }
}

pub fn value_to_words(
    value: Number,
    dictionary: &Dictionary,
    ruleset: &LanguageRuleset,
) -> Option<NumberRepresentation> {
    ruleset.value_to_words(value, dictionary)
}

#[cfg(test)]
mod tests {
    use crate::language::{
        Dictionary,
        counter::{
            DutchRules, EnglishRules, LanguageRuleset, NumberRepresentation, value_to_words,
        },
        dictionary::DictionaryConstant,
    };

    fn create_dictionary_english() -> Dictionary {
        Dictionary {
            constants: vec![
                DictionaryConstant {
                    value: 0,
                    representation: "zero".to_string(),
                    number_of_syllables: 2,
                },
                DictionaryConstant {
                    value: 1,
                    representation: "one".to_string(),
                    number_of_syllables: 1,
                },
                DictionaryConstant {
                    value: 2,
                    representation: "two".to_string(),
                    number_of_syllables: 1,
                },
                DictionaryConstant {
                    value: 3,
                    representation: "three".to_string(),
                    number_of_syllables: 1,
                },
                DictionaryConstant {
                    value: 4,
                    representation: "four".to_string(),
                    number_of_syllables: 1,
                },
                DictionaryConstant {
                    value: 10,
                    representation: "ten".to_string(),
                    number_of_syllables: 1,
                },
                DictionaryConstant {
                    value: 20,
                    representation: "twenty".to_string(),
                    number_of_syllables: 2,
                },
                DictionaryConstant {
                    value: 40,
                    representation: "forty".to_string(),
                    number_of_syllables: 2,
                },
                DictionaryConstant {
                    value: 100,
                    representation: "hundred".to_string(),
                    number_of_syllables: 2,
                },
                DictionaryConstant {
                    value: 1000,
                    representation: "thousand".to_string(),
                    number_of_syllables: 2,
                },
            ],
            operators: vec![],
        }
    }

    fn create_dictionary_dutch() -> Dictionary {
        Dictionary {
            constants: vec![
                DictionaryConstant {
                    value: 0,
                    representation: "nul".to_string(),
                    number_of_syllables: 1,
                },
                DictionaryConstant {
                    value: 1,
                    representation: "een".to_string(),
                    number_of_syllables: 1,
                },
                DictionaryConstant {
                    value: 2,
                    representation: "twee".to_string(),
                    number_of_syllables: 1,
                },
                DictionaryConstant {
                    value: 3,
                    representation: "drie".to_string(),
                    number_of_syllables: 1,
                },
                DictionaryConstant {
                    value: 4,
                    representation: "vier".to_string(),
                    number_of_syllables: 1,
                },
                DictionaryConstant {
                    value: 10,
                    representation: "tien".to_string(),
                    number_of_syllables: 1,
                },
                DictionaryConstant {
                    value: 20,
                    representation: "twintig".to_string(),
                    number_of_syllables: 2,
                },
                DictionaryConstant {
                    value: 40,
                    representation: "veertig".to_string(),
                    number_of_syllables: 2,
                },
                DictionaryConstant {
                    value: 100,
                    representation: "honderd".to_string(),
                    number_of_syllables: 2,
                },
                DictionaryConstant {
                    value: 1000,
                    representation: "duizend".to_string(),
                    number_of_syllables: 2,
                },
            ],
            operators: vec![],
        }
    }

    #[test]
    fn test_value_to_word_english() {
        let dictionary = create_dictionary_english();
        let ruleset = LanguageRuleset::English(EnglishRules);
        assert_eq!(
            value_to_words(3, &dictionary, &ruleset),
            Some(NumberRepresentation {
                value: 3,
                representation: "three".to_owned(),
                number_of_syllables: 1
            })
        );
        assert_eq!(
            value_to_words(43, &dictionary, &ruleset),
            Some(NumberRepresentation {
                value: 43,
                representation: "forty three".to_owned(),
                number_of_syllables: 3
            })
        );
        assert_eq!(
            value_to_words(420, &dictionary, &ruleset),
            Some(NumberRepresentation {
                value: 420,
                representation: "four hundred twenty".to_owned(),
                number_of_syllables: 5
            })
        );
        assert_eq!(
            value_to_words(2003, &dictionary, &ruleset),
            Some(NumberRepresentation {
                value: 2003,
                representation: "two thousand three".to_owned(),
                number_of_syllables: 1 + 2 + 1,
            })
        );
    }

    #[test]
    fn test_value_to_word_dutch() {
        let dictionary = create_dictionary_dutch();
        let ruleset = LanguageRuleset::Dutch(DutchRules);
        assert_eq!(
            value_to_words(3, &dictionary, &ruleset),
            Some(NumberRepresentation {
                value: 3,
                representation: "drie".to_owned(),
                number_of_syllables: 1
            })
        );

        // TODO: should actually be testing for "drieënveertig", but the
        // trema rule is not implemented yet.
        assert_eq!(
            value_to_words(43, &dictionary, &ruleset),
            Some(NumberRepresentation {
                value: 43,
                representation: "drieenveertig".to_owned(),
                number_of_syllables: 4
            })
        );
        assert_eq!(
            value_to_words(420, &dictionary, &ruleset),
            Some(NumberRepresentation {
                value: 420,
                representation: "vierhonderdtwintig".to_owned(),
                number_of_syllables: 5
            })
        );
        assert_eq!(
            value_to_words(2003, &dictionary, &ruleset),
            Some(NumberRepresentation {
                value: 2003,
                representation: "tweeduizend drie".to_owned(),
                number_of_syllables: 1 + 2 + 1,
            })
        );
    }
}
