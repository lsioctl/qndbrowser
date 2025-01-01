use std::io::LineWriter;

#[derive(Debug)]
enum MathExpr {
    Plus {
        left: Box<MathExpr>,
        right: Box<MathExpr>,
    },
    Minus {
        left: Box<MathExpr>,
        right: Box<MathExpr>,
    },
    Literal {
        literal: i64,
    },
}

impl MathExpr {
    fn eval(&self) -> i64 {
        match self {
            MathExpr::Plus { left, right } => left.eval() + right.eval(),
            MathExpr::Minus { left, right } => left.eval() - right.eval(),
            MathExpr::Literal { literal } => *literal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_test() {
        let expr = MathExpr::Plus {
            left: Box::new(MathExpr::Literal { literal: 2 }),
            right: Box::new(MathExpr::Minus {
                left: Box::new(MathExpr::Literal { literal: 5 }),
                right: Box::new(MathExpr::Literal { literal: 1 }),
            }),
        };

        // println will work only with nocapture
        // cargo test -- --nocapture
        println!("{:?}", expr);
        // assert_eq!(result, 4);
    }

    #[test]
    fn eval_first_test() {
        let expr = MathExpr::Plus {
            left: Box::new(MathExpr::Literal { literal: 2 }),
            right: Box::new(MathExpr::Minus {
                left: Box::new(MathExpr::Literal { literal: 6 }),
                right: Box::new(MathExpr::Literal { literal: 3 }),
            }),
        };

        assert_eq!(expr.eval(), 5);
    }
}
