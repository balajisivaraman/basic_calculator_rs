use crate::types::Expr;
use crate::types::Expr::*;

pub(crate) fn evaluate(expr: Expr) -> i32 {
    match expr {
        ENum(num) => num,
        EAdd(expr1, expr2) => evaluate(*expr1) + evaluate(*expr2),
        ESub(expr1, expr2) => evaluate(*expr1) - evaluate(*expr2),
    }
}

#[cfg(test)]
mod tests {
    use crate::evaluator::evaluate;
    use crate::types::Expr::*;

    #[test]
    fn evaluate_enum_test() {
        let expr = ENum(1234);
        assert_eq!(evaluate(expr), 1234);
    }

    #[test]
    fn evaluate_eadd_test() {
        let expr = EAdd(Box::new(ENum(12)), Box::new(ENum(34)));
        assert_eq!(evaluate(expr), 46);
    }

    #[test]
    fn evaluate_easub_test() {
        let expr = ESub(Box::new(ENum(12)), Box::new(ENum(34)));
        assert_eq!(evaluate(expr), -22);
    }
}
