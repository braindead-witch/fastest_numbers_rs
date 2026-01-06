#[cfg(test)]
mod tests {
    use crate::{math::math::Expression, syllables::{Dictionary, counter::{NumberRepresentation, value_to_words}}};

    #[test]
    fn test_tree_eval_value() {
        let expr1 = Expression::Constant(3);
        let expr2 = Expression::Squared(Box::new(expr1.clone()));
        let expr3 = Expression::Multiply(Box::new(expr2.clone()), Box::new(Expression::Constant(5)));
        let expr4 = Expression::Divide(Box::new(expr3.clone()), Box::new(Expression::Constant(3)));
        let expr4_illegal_divide = Expression::Divide(Box::new(expr3.clone()), Box::new(Expression::Constant(7)));
        let expr5 = Expression::Exponent(Box::new(expr1.clone()), Box::new(Expression::Constant(3)));

        assert_eq!(expr1.eval(), Ok(3));
        assert_eq!(expr2.eval(), Ok(9));
        assert_eq!(expr3.eval(), Ok(45));
        assert_eq!(expr4.eval(), Ok(15));
        assert!(expr4_illegal_divide.eval().is_err());
        assert_eq!(expr5.eval(), Ok(27))
    }

    #[test]
    fn test_tree_to_string() {
        let expr1 = Expression::Constant(3);
        let expr2 = Expression::Squared(Box::new(expr1.clone()));
        let expr3 = Expression::Multiply(Box::new(expr2.clone()), Box::new(Expression::Constant(5)));
        let expr4 = Expression::Divide(Box::new(expr3.clone()), Box::new(Expression::Constant(3)));
        let expr4_illegal_divide = Expression::Divide(Box::new(expr3.clone()), Box::new(Expression::Constant(7)));
        let expr5 = Expression::Exponent(Box::new(expr1.clone()), Box::new(Expression::Constant(3)));

        // assert_eq!(expr1.to_string(), "three");
        // assert_eq!(expr2.to_string(), "three squared");
        // assert_eq!(expr3.to_string(), "three squared times 5");
        // assert_eq!(expr4.to_string(), "three squared times 5 over 3");
        // assert_eq!(expr4_illegal_divide.to_string(), "three squared times 5 over seven");
        // assert_eq!(expr5.to_string(), "three to the three");
    }

    #[test]
    fn test_value_to_word() {
        let dictionary = Dictionary::from_file("en-gb.json");
        assert_eq!(
            value_to_words(5, &dictionary), 
            Some(NumberRepresentation { representation: "five".to_owned(), number_of_syllables: 1 })
        );
        assert_eq!(
            value_to_words(45, &dictionary), 
            Some(NumberRepresentation { representation: "forty five".to_owned(), number_of_syllables: 3 })
        );
        assert_eq!(
            value_to_words(420, &dictionary), 
            Some(NumberRepresentation { representation: "four hundred twenty".to_owned(), number_of_syllables: 5 })
        );
        assert_eq!(
            value_to_words(2147483647, &dictionary), 
            Some(NumberRepresentation { 
                representation: "two billion one hundred forty seven million four hundred eighty three thousand six hundred forty seven".to_owned(),
                number_of_syllables: 1+2+1+2+2+2+2+1+2+2+1+2+1+2+2+2,
            })
        );
    }
}