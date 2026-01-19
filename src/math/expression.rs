// Contains all functions for math operations
use crate::syllables::{Dictionary, counter::value_to_words};
use serde::Deserialize;

pub type Number = i32;

#[derive(Debug, PartialEq)]
pub enum ArithmeticError {
    DivideByZero(String),
    NegativeExponent(String),
    OverflowError(String),
    NonIntegerSolution(String),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Atom(Number),
    Operation(Operator),
}

#[derive(Debug, Clone)]
pub enum Operator {
    Unary(UnaryOperator, Box<Expression>),
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
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
                None => Err(ArithmeticError::OverflowError(format!("Overflow occurred trying to calculate {} squared", x))),
            }
            UnaryOperator::Cubed => match x.checked_pow(3) {
                Some(v) => Ok(v),
                None => Err(ArithmeticError::OverflowError(format!("Overflow occurred trying to calculate {} cubed", x))),
            }
        }
    }

    pub fn get_inverse(&self, x: Number) -> Result<Number, ArithmeticError> {
        match self {
            UnaryOperator::Squared => {
                let lhs = x.isqrt();
                if lhs * lhs != x {
                    Err(ArithmeticError::NonIntegerSolution(format!("{}^2 != {}", lhs, x)))
                } else {
                    Ok(lhs)
                }
            },
            UnaryOperator::Cubed => {
                let lhs = (x as f64).powf(1.0 / 3.0) as Number; // Try to take cube root
                if lhs * lhs * lhs != x {
                    Err(ArithmeticError::NonIntegerSolution(format!("{}^3 != {}", lhs, x)))
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
                None => Err(ArithmeticError::OverflowError(format!("Overflow occurred trying to calculate {}*{}", x, y))),
            },
            BinaryOperator::Divide => Self::checked_divide(x, y),
            BinaryOperator::Exponent => {
                if y < 0 {
                    Err(ArithmeticError::NegativeExponent(format!("Negative exponent not supported: {}^{}", x, y)))
                } else {
                    match x.checked_pow(y.try_into().unwrap()) {
                        Some(v) => Ok(v),
                        None => Err(ArithmeticError::OverflowError(format!("Overflow occurred trying to calculate {}^{}", x, y))),
                    }
                }
            },
        }
    }

    pub fn get_rhs(&self, result: Number, lhs: Number) -> Result<Number, ArithmeticError> {
        match self {
            BinaryOperator::Add => Ok(result - lhs),
            BinaryOperator::Subtract => Ok(lhs - result), // lhs - rhs = result
            BinaryOperator::Multiply => Self::checked_divide(result, lhs), // rhs = result/lhs
            BinaryOperator::Divide => { // lhs / rhs = result => rhs = lhs / result
                if lhs == 0 {
                    Err(ArithmeticError::DivideByZero(format!("Will result in a division by zero: {lhs}/rhs = {result}")))
                } else {
                    Self::checked_divide(lhs, result)
                }
            },
            BinaryOperator::Exponent => {
                // lhs ^ rhs = result
                // rhs * log(lhs) = log(result)
                // rhs = log(result)/log(lhs)
                if result <= 0 || lhs <= 0 {
                    return Err(ArithmeticError::NegativeExponent(format!("Can't take logarithm of a negative value, or zero: {lhs}^x={result}")));
                }
                let log_lhs = lhs.ilog2();
                if log_lhs == 0 {
                    return Err(ArithmeticError::DivideByZero(format!("Attempted to divide by zero: {lhs}^x = {result}")));
                }
                let rhs = (result.ilog2() / log_lhs) as Number;
                if lhs.pow(rhs.try_into().unwrap()) == result {
                    Ok(rhs)
                } else {
                    Err(ArithmeticError::NonIntegerSolution(format!("{}^{} != {}", lhs, rhs, result)))
                }
            }
        }
    }

    fn checked_divide(x: Number, y: Number) -> Result<Number, ArithmeticError> {
        if y == 0 {
            Err(ArithmeticError::DivideByZero(
                format!("Division by zero: {} / {}", x, y))
            )
        } else if (x % y) != 0 {
            Err(ArithmeticError::NonIntegerSolution(
                format!("Non-integer division: {} / {}", x, y))
            )
        } else {
            Ok(x / y)
        }
    }
}

impl Expression {
    pub fn eval(&self) -> Result<Number, ArithmeticError> {
        match self {
            Expression::Atom(x) => Ok(*x),
            Expression::Operation(op) => match op {
                Operator::Unary(op, x) => {
                    op.apply(x.eval()?)
                },
                Operator::Binary(op, x, y) => {
                    op.apply(x.eval()?, y.eval()?)
                }
            }
        }
    }

    pub fn number_of_syllables(&self, dictionary: &Dictionary) -> Number {
        match self {
            Expression::Atom(x) => {
                if let Some(item) = value_to_words(*x, dictionary) {
                    item.number_of_syllables
                } else {
                    panic!("Couldn't convert to words: {}", x);
                }
            },
            Expression::Operation(op) => {
                // Sum of LHS + Operator + RHS
                let number_of_syllables_operator = if let Some(item) = dictionary.get_from_operator(op) {
                    item.number_of_syllables
                } else {
                    panic!("Operator {:#?} not in dictionary", op);
                };

                match op {
                    Operator::Unary(_, x) => number_of_syllables_operator + x.number_of_syllables(dictionary),
                    Operator::Binary(_, x, y) => number_of_syllables_operator + x.number_of_syllables(dictionary) + y.number_of_syllables(dictionary),
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{math::expression::{Expression, Operator, BinaryOperator, UnaryOperator}, syllables::Dictionary};

    fn make_expressions() -> (Expression, Expression, Expression, Expression, Expression, Expression) {
        let expr1 = Expression::Atom(3);
        let expr2 = Expression::Operation(Operator::Unary(
            UnaryOperator::Squared,
            Box::new(expr1.clone())
        ));
        let expr3 = Expression::Operation(Operator::Binary(
            BinaryOperator::Multiply,
            Box::new(expr2.clone()),
            Box::new(Expression::Atom(5))
        ));
        let expr4 = Expression::Operation(Operator::Binary(
            BinaryOperator::Divide,
            Box::new(expr3.clone()),
            Box::new(Expression::Atom(3))
        ));
        let expr4_illegal_divide = Expression::Operation(Operator::Binary(
            BinaryOperator::Divide,
            Box::new(expr3.clone()),
            Box::new(Expression::Atom(7))
        ));
        let expr5 = Expression::Operation(Operator::Binary(
            BinaryOperator::Exponent,
            Box::new(expr1.clone()),
            Box::new(Expression::Atom(3))
        ));

        (expr1, expr2, expr3, expr4, expr4_illegal_divide, expr5)
    }

    #[test]
    fn test_tree_eval_value() {
        let (expr1, expr2, expr3, expr4, expr4_illegal_divide, expr5) = make_expressions();

        assert_eq!(expr1.eval(), Ok(3));
        assert_eq!(expr2.eval(), Ok(9));
        assert_eq!(expr3.eval(), Ok(45));
        assert_eq!(expr4.eval(), Ok(15));
        assert!(expr4_illegal_divide.eval().is_err());
        assert_eq!(expr5.eval(), Ok(27))
    }

    #[test]
    fn test_tree_to_string() {
        todo!();
        let (expr1, expr2, expr3, expr4, expr4_illegal_divide, expr5) = make_expressions();

        // assert_eq!(expr1.to_string(), "three");
        // assert_eq!(expr2.to_string(), "three squared");
        // assert_eq!(expr3.to_string(), "three squared times 5");
        // assert_eq!(expr4.to_string(), "three squared times 5 over 3");
        // assert_eq!(expr4_illegal_divide.to_string(), "three squared times 5 over seven");
        // assert_eq!(expr5.to_string(), "three to the three");
    }

    #[test]
    fn test_tree_count_syllables() {
        let dictionary = Dictionary::from_file("en-gb.json");

        let (expr1, expr2, expr3, expr4, expr4_illegal_divide, expr5) = make_expressions();

        assert_eq!(expr1.number_of_syllables(&dictionary), 1);
        assert_eq!(expr2.number_of_syllables(&dictionary), 2);
        assert_eq!(expr3.number_of_syllables(&dictionary), 4);
        assert_eq!(expr4.number_of_syllables(&dictionary), 6);
        assert_eq!(expr4_illegal_divide.number_of_syllables(&dictionary), 7);
        assert_eq!(expr5.number_of_syllables(&dictionary), 4);
    }
}

