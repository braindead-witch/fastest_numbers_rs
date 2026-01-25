use serde::Deserialize;

pub type Number = i32;

#[derive(Debug, PartialEq)]
pub enum ArithmeticError {
    DivideByZero(String),
    NegativeExponent(String),
    OverflowError(String),
    NonIntegerSolution(String),
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UnaryOperator {
    Squared,
    Cubed,
}

impl UnaryOperator {
    pub fn apply(&self, x: Number) -> Result<Number, ArithmeticError> {
        match self {
            UnaryOperator::Squared => match x.checked_mul(x) {
                Some(v) => Ok(v),
                None => Err(ArithmeticError::OverflowError(format!(
                    "Overflow occurred trying to calculate {} squared",
                    x
                ))),
            },
            UnaryOperator::Cubed => match x.checked_pow(3) {
                Some(v) => Ok(v),
                None => Err(ArithmeticError::OverflowError(format!(
                    "Overflow occurred trying to calculate {} cubed",
                    x
                ))),
            },
        }
    }

    pub fn get_inverse(&self, x: Number) -> Result<Number, ArithmeticError> {
        match self {
            UnaryOperator::Squared => {
                let lhs = x.isqrt();
                if lhs * lhs != x {
                    Err(ArithmeticError::NonIntegerSolution(format!(
                        "{}^2 != {}",
                        lhs, x
                    )))
                } else {
                    Ok(lhs)
                }
            },
            UnaryOperator::Cubed => {
                let lhs = (x as f64).powf(1.0 / 3.0) as Number; // Try to take cube root
                if lhs * lhs * lhs != x {
                    Err(ArithmeticError::NonIntegerSolution(format!(
                        "{}^3 != {}",
                        lhs, x
                    )))
                } else {
                    Ok(lhs)
                }
            },
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
}

impl BinaryOperator {
    pub fn apply(&self, x: Number, y: Number) -> Result<Number, ArithmeticError> {
        match self {
            BinaryOperator::Add => Ok(x + y),
            BinaryOperator::Subtract => Ok(x - y),
            BinaryOperator::Multiply => match x.checked_mul(y) {
                Some(v) => Ok(v),
                None => Err(ArithmeticError::OverflowError(format!(
                    "Overflow occurred trying to calculate {}*{}",
                    x, y
                ))),
            },
            BinaryOperator::Divide => Self::checked_divide(x, y),
            BinaryOperator::Exponent => {
                if y < 0 {
                    Err(ArithmeticError::NegativeExponent(format!(
                        "Negative exponent not supported: {}^{}",
                        x, y
                    )))
                } else {
                    match x.checked_pow(y.try_into().unwrap()) {
                        Some(v) => Ok(v),
                        None => Err(ArithmeticError::OverflowError(format!(
                            "Overflow occurred trying to calculate {}^{}",
                            x, y
                        ))),
                    }
                }
            }
        }
    }

    pub fn is_commutative(&self) -> bool {
        match self {
            BinaryOperator::Add => true,
            BinaryOperator::Subtract => false,
            BinaryOperator::Multiply => true,
            BinaryOperator::Divide => false,
            BinaryOperator::Exponent => false,
        }
    }

    pub fn get_rhs(&self, result: Number, lhs: Number) -> Result<Number, ArithmeticError> {
        match self {
            BinaryOperator::Add => Ok(result - lhs),
            BinaryOperator::Subtract => Ok(lhs - result), // lhs - rhs = result
            BinaryOperator::Multiply => Self::checked_divide(result, lhs), // rhs = result/lhs
            BinaryOperator::Divide => {
                // lhs / rhs = result => rhs = lhs / result
                if lhs == 0 {
                    Err(ArithmeticError::DivideByZero(format!(
                        "Will result in a division by zero: {lhs}/rhs = {result}"
                    )))
                } else {
                    Self::checked_divide(lhs, result)
                }
            }
            BinaryOperator::Exponent => {
                // lhs ^ rhs = result
                // rhs * log(lhs) = log(result)
                // rhs = log(result)/log(lhs)
                if result <= 0 || lhs <= 0 {
                    return Err(ArithmeticError::NegativeExponent(format!(
                        "Can't take logarithm of a negative value, or zero: {lhs}^x={result}"
                    )));
                }
                let log_lhs = lhs.ilog2();
                if log_lhs == 0 {
                    return Err(ArithmeticError::DivideByZero(format!(
                        "Attempted to divide by zero: {lhs}^x = {result}"
                    )));
                }
                let rhs = (result.ilog2() / log_lhs) as Number;
                if lhs.pow(rhs.try_into().unwrap()) == result {
                    Ok(rhs)
                } else {
                    Err(ArithmeticError::NonIntegerSolution(format!(
                        "{}^{} != {}",
                        lhs, rhs, result
                    )))
                }
            }
        }
    }

    fn checked_divide(x: Number, y: Number) -> Result<Number, ArithmeticError> {
        if y == 0 {
            Err(ArithmeticError::DivideByZero(format!(
                "Division by zero: {} / {}",
                x, y
            )))
        } else if (x % y) != 0 {
            Err(ArithmeticError::NonIntegerSolution(format!(
                "Non-integer division: {} / {}",
                x, y
            )))
        } else {
            Ok(x / y)
        }
    }
}

#[cfg(test)]
mod tests { }
