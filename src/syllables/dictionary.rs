use crate::math::expression::{self, BinaryOperator, Number, UnaryOperator};
use serde::Deserialize;
use std::{fs::File, io::BufReader, path::Path};

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Operator {
    Unary(UnaryOperator),
    Binary(BinaryOperator),
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DictionaryConstant {
    pub representation: String,
    pub value: Number,
    pub number_of_syllables: Number,
}

impl PartialOrd for DictionaryConstant {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DictionaryConstant {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct DictionaryOperator {
    pub representation: String,
    pub operator: Operator,
    pub number_of_syllables: Number,
}

#[derive(Deserialize, Debug)]
pub struct Dictionary {
    pub constants: Vec<DictionaryConstant>,
    pub inverse_constants: Vec<DictionaryConstant>,
    pub operators: Vec<DictionaryOperator>,
}

impl Dictionary {
    pub fn from_file<P: AsRef<Path>>(filepath: P) -> Dictionary {
        let file = File::open(filepath).unwrap();
        let reader = BufReader::new(file);
        let contents = serde_json::from_reader(reader);

        contents.unwrap()
    }

    pub fn get_from_value(&self, value: Number) -> Option<DictionaryConstant> {
        // Linear search
        // Could change `constants` to a HashMap if performance is needed in the future
        for item in &self.constants {
            if item.value == value {
                return Some(item.clone());
            }
        }
        None
    }

    pub fn get_from_operator(&self, operator: &expression::Operator) -> Option<DictionaryOperator> {
        match operator {
            expression::Operator::Binary(op, _, _) => {
                match op {
                    expression::BinaryOperator::Add => {
                        self.get_from_dictionary_operator(Operator::Binary(BinaryOperator::Add))
                    }
                    expression::BinaryOperator::Subtract => self
                        .get_from_dictionary_operator(Operator::Binary(BinaryOperator::Subtract)),
                    expression::BinaryOperator::Multiply => self
                        .get_from_dictionary_operator(Operator::Binary(BinaryOperator::Multiply)),
                    expression::BinaryOperator::Divide => {
                        self.get_from_dictionary_operator(Operator::Binary(BinaryOperator::Divide))
                    }
                    expression::BinaryOperator::Exponent => self
                        .get_from_dictionary_operator(Operator::Binary(BinaryOperator::Exponent)),
                }
            }
            expression::Operator::Unary(op, _) => match op {
                expression::UnaryOperator::Squared => {
                    self.get_from_dictionary_operator(Operator::Unary(UnaryOperator::Squared))
                }
                expression::UnaryOperator::Cubed => {
                    self.get_from_dictionary_operator(Operator::Unary(UnaryOperator::Cubed))
                }
            },
        }
    }

    pub fn get_from_dictionary_operator(&self, operator: Operator) -> Option<DictionaryOperator> {
        self.operators
            .iter()
            .find(|op| op.operator == operator)
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use crate::syllables::{
        Dictionary,
        counter::{NumberRepresentation, value_to_words},
    };

    #[test]
    fn test_value_to_word() {
        let dictionary = Dictionary::from_file("en-gb.json");
        assert_eq!(
            value_to_words(5, &dictionary),
            Some(NumberRepresentation {
                value: 5,
                representation: "five".to_owned(),
                number_of_syllables: 1
            })
        );
        assert_eq!(
            value_to_words(45, &dictionary),
            Some(NumberRepresentation {
                value: 45,
                representation: "forty five".to_owned(),
                number_of_syllables: 3
            })
        );
        assert_eq!(
            value_to_words(420, &dictionary),
            Some(NumberRepresentation {
                value: 420,
                representation: "four hundred twenty".to_owned(),
                number_of_syllables: 5
            })
        );
        assert_eq!(
            value_to_words(2147483647, &dictionary),
            Some(NumberRepresentation {
                value: 2147483647,
                representation: "two billion one hundred forty seven million four hundred eighty three thousand six hundred forty seven".to_owned(),
                number_of_syllables: 1+2+1+2+2+2+2+1+2+2+1+2+1+2+2+2,
            })
        );
    }
}
