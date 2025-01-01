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
    Litteral {
        litteral: i64,
    },
}

impl MathExpr {
    fn eval(&self) -> i64 {
        match self {
            MathExpr::Plus { left, right } => left.eval() + right.eval(),
            MathExpr::Minus { left, right } => left.eval() - right.eval(),
            MathExpr::Litteral { litteral } => *litteral,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_test() {
        let expr = MathExpr::Plus {
            left: Box::new(MathExpr::Litteral { litteral: 2 }),
            right: Box::new(MathExpr::Minus {
                left: Box::new(MathExpr::Litteral { litteral: 5 }),
                right: Box::new(MathExpr::Litteral { litteral: 1 }),
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
            left: Box::new(MathExpr::Litteral { litteral: 2 }),
            right: Box::new(MathExpr::Minus {
                left: Box::new(MathExpr::Litteral { litteral: 6 }),
                right: Box::new(MathExpr::Litteral { litteral: 3 }),
            }),
        };

        assert_eq!(expr.eval(), 5);
    }
}
