use crate::types::Expr;
use crate::types::Expr::*;

pub(crate) fn evaluate(expr: Expr) -> f64 {
    match expr {
        Num(num) => num,
        Add(expr1, expr2) => evaluate(*expr1) + evaluate(*expr2),
        Sub(expr1, expr2) => evaluate(*expr1) - evaluate(*expr2),
        Mul(expr1, expr2) => evaluate(*expr1) * evaluate(*expr2),
        Div(expr1, expr2) => evaluate(*expr1) / evaluate(*expr2),
        Exp(expr1, expr2) => evaluate(*expr1).powf(evaluate(*expr2)),
        Sin(expr1) => evaluate(*expr1).sin(),
        Cos(expr1) => evaluate(*expr1).cos(),
    }
}

#[cfg(test)]
mod tests {
    use crate::evaluator::evaluate;
    use crate::types::Expr::*;

    #[test]
    fn evaluate_enum_test() {
        let expr = Num(1234.0);
        assert_eq!(evaluate(expr), 1234.0);
    }

    #[test]
    fn evaluate_eadd_test() {
        let expr = Add(Box::new(Num(12.0)), Box::new(Num(34.0)));
        assert_eq!(evaluate(expr), 46.0);
    }

    #[test]
    fn evaluate_easub_test() {
        let expr = Sub(Box::new(Num(12.0)), Box::new(Num(34.0)));
        assert_eq!(evaluate(expr), -22.0);
    }

    #[test]
    fn test_evaluate_nested_arithmetic_expression() {
        let expr = Add(
            Box::new(Mul(Box::new(Num(1.0)), Box::new(Num(2.0)))),
            Box::new(Div(
                Box::new(Exp(Box::new(Num(6.0)), Box::new(Num(2.0)))),
                Box::new(Num(5.0)),
            )),
        );
        assert_eq!(evaluate(expr), 9.2);
    }
}
