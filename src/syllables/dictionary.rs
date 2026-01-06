use std::{fs::File, io::BufReader, path::Path};
use serde::Deserialize;
use crate::math::math::Number;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Operator {
    // Unary
    Squared,
    Cubed,

    // Binary
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DictionaryConstant {
    pub representation: String,
    pub value: Number,
    pub number_of_syllables: u32,
}

impl PartialOrd for DictionaryConstant {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for DictionaryConstant {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

#[derive(Deserialize, Debug)]
pub struct DictionaryOperator {
    pub representation: String,
    pub operator: Operator,
    pub number_of_syllables: u32,
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
}
