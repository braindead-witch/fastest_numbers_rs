use crate::math::expression::{BinaryOperator, Number, UnaryOperator};
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

    pub fn get_from_operator(&self, operator: Operator) -> Option<DictionaryOperator> {
        self.operators
            .iter()
            .find(|op| op.operator == operator)
            .cloned()
    }
}

#[cfg(test)]
mod tests {}
