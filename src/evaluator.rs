use crate::types::Expr;
use crate::types::Expr::*;

pub(crate) fn evaluate(expr: Expr) -> f32 {
    match expr {
        ENum(num) => num,
        EAdd(expr1, expr2) => evaluate(*expr1) + evaluate(*expr2),
        ESub(expr1, expr2) => evaluate(*expr1) - evaluate(*expr2),
        EMul(expr1, expr2) => evaluate(*expr1) * evaluate(*expr2),
        EDiv(expr1, expr2) => evaluate(*expr1) / evaluate(*expr2),
        EExp(expr1, expr2) => evaluate(*expr1).powf(evaluate(*expr2)),
    }
}

#[cfg(test)]
mod tests {
    use crate::evaluator::evaluate;
    use crate::types::Expr::*;

    #[test]
    fn evaluate_enum_test() {
        let expr = ENum(1234.0);
        assert_eq!(evaluate(expr), 1234.0);
    }

    #[test]
    fn evaluate_eadd_test() {
        let expr = EAdd(Box::new(ENum(12.0)), Box::new(ENum(34.0)));
        assert_eq!(evaluate(expr), 46.0);
    }

    #[test]
    fn evaluate_easub_test() {
        let expr = ESub(Box::new(ENum(12.0)), Box::new(ENum(34.0)));
        assert_eq!(evaluate(expr), -22.0);
    }

    #[test]
    fn test_evaluate_nested_arithmetic_expression() {
        let expr = EAdd(
            Box::new(EMul(Box::new(ENum(1.0)), Box::new(ENum(2.0)))),
            Box::new(EDiv(
                Box::new(EExp(Box::new(ENum(6.0)), Box::new(ENum(2.0)))),
                Box::new(ENum(5.0)),
            )),
        );
        assert_eq!(evaluate(expr), 9.2);
    }
}
