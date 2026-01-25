use std::fmt::Display;

use crate::math::{export::OptimizedResult, expression::Number};

pub struct Statistics {
    min_number: Option<Number>,
    max_number: Option<Number>,
    max_syllables: Option<Number>,
    average_syllables: f32,
}

impl Statistics {
    pub fn from_optimization_result(results: OptimizedResult) -> Self {
        Statistics {
            min_number: results.inner.iter().min_by_key(|k| k.0).map(|opt| *opt.0),
            max_number: results.inner.iter().max_by_key(|k| k.0).map(|opt| *opt.0),
            max_syllables: results
                .inner
                .iter()
                .max_by_key(|k| k.1.number_of_syllables)
                .map(|opt| opt.1.number_of_syllables),
            average_syllables: {
                let sum: Number = results.inner.iter().map(|k| k.1.number_of_syllables).sum();
                let count = results.inner.len();
                (sum as f32) / (count as f32)
            },
        }
    }
}

impl Display for Statistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Number range: [{:?}, {:?}]\nMax amount of syllables: {:?}\nAverage amount of syllables: {:?}",
            self.min_number, self.max_number, self.max_syllables, self.average_syllables
        )
    }
}
