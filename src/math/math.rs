// Contains all functions for math operations

pub type Number = i32;

#[derive(Debug, PartialEq)]
pub enum ArithmeticError {
    DivideByZero(String),
    DivisionWithRemainder(String),
    NegativeExponent(String)
}

#[derive(Clone)]
pub enum Expression {
    Constant(Number),

    // Unary
    Squared(Box<Expression>),
    Cubed(Box<Expression>),

    // Binary
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Exponent(Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn eval(&self) -> Result<Number, ArithmeticError> {
        match self {
            Expression::Constant(x) => Ok(*x),
            Expression::Squared(x) => Ok(x.eval()? * x.eval()?),
            Expression::Cubed(x) => Ok(x.eval()? * x.eval()? * x.eval()?),
            Expression::Add(x, y) => Ok(x.eval()? + y.eval()?),
            Expression::Subtract(x, y) => Ok(x.eval()? - y.eval()?),
            Expression::Multiply(x, y) => Ok(x.eval()? * y.eval()?),
            Expression::Divide(x, y) => {
                let x_value = x.eval()?;
                let y_value = y.eval()?;
                if y_value == 0 {
                    Err(ArithmeticError::DivideByZero(
                        format!("Division by zero: {} / {}", x_value, y_value))
                    )
                } else if (x_value % y_value) != 0 {
                    Err(ArithmeticError::DivisionWithRemainder(
                        format!("Non-integer division: {} / {}", x_value, y_value))
                    )
                } else {
                    Ok(x_value / y_value)
                }
            },
            Expression::Exponent(x, y) => {
                let x_value = x.eval()?;
                let y_value = y.eval()?;
                if y_value < 0 {
                    Err(ArithmeticError::NegativeExponent(format!("Negative exponent not supported: {}^{}", x_value, y_value)))
                } else {
                    Ok(x_value.pow(y_value.try_into().unwrap()))
                }
            },
        }
    }
}
