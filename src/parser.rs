use crate::types::Expr;
use crate::types::Expr::*;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, space0};
use nom::number::complete::{f64, double};
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use nom::IResult;
use std::str::FromStr;

pub(crate) fn parse(input: &str) -> IResult<&str, Expr> {
    parse_basic_expr(input)
}

fn parse_basic_expr(input: &str) -> IResult<&str, Expr> {
    parse_math_expr(input)
}

fn parse_parens(input: &str) -> IResult<&str, Expr> {
    delimited(
        space0,
        delimited(char('('), parse_math_expr, char(')')),
        space0,
    )(input)
}

fn parse_function_sin(input: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            tag("sin"),
            delimited(char('('), parse_math_expr, char(')')),
        )),
        |(_, expr)| Sin(Box::new(expr)),
    )(input)
}

fn parse_function_cos(input: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            tag("cos"),
            delimited(char('('), parse_math_expr, char(')')),
        )),
        |(_, expr)| Cos(Box::new(expr)),
    )(input)
}

fn parse_function(input: &str) -> IResult<&str, Expr> {
    alt((parse_function_sin, parse_function_cos))(input)
}

fn parse_operation(input: &str) -> IResult<&str, Expr> {
    alt((parse_parens, parse_number, parse_function))(input)
}

fn parse_factor(input: &str) -> IResult<&str, Expr> {
    let (input, num1) = parse_operation(input)?;
    let (input, exprs) = many0(tuple((char('^'), parse_factor)))(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_term(input: &str) -> IResult<&str, Expr> {
    let (input, num1) = parse_factor(input)?;
    let (input, exprs) = many0(tuple((alt((char('/'), char('*'))), parse_factor)))(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_math_expr(input: &str) -> IResult<&str, Expr> {
    let (input, num1) = parse_term(input)?;
    let (input, exprs) = many0(tuple((alt((char('+'), char('-'))), parse_term)))(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_expr(expr: Expr, rem: Vec<(char, Expr)>) -> Expr {
    rem.into_iter().fold(expr, |acc, val| parse_op(val, acc))
}

fn parse_op(tup: (char, Expr), expr1: Expr) -> Expr {
    let (op, expr2) = tup;
    match op {
        '+' => Add(Box::new(expr1), Box::new(expr2)),
        '-' => Sub(Box::new(expr1), Box::new(expr2)),
        '*' => Mul(Box::new(expr1), Box::new(expr2)),
        '/' => Div(Box::new(expr1), Box::new(expr2)),
        '^' => Exp(Box::new(expr1), Box::new(expr2)),
        _ => panic!("Unknown Operation"),
    }
}

fn parse_number(input: &str) -> IResult<&str, Expr> {
    map(
        delimited(space0, double, space0),
        |num| Num(num),
    )(input)}

#[cfg(test)]
mod tests {
    use crate::parser::parse;
    use crate::types::Expr::*;

    #[test]
    fn parse_add_statement() {
        let parsed = parse("12 + 34");
        assert_eq!(
            parsed,
            Ok(("", Add(Box::new(Num(12.0)), Box::new(Num(34.0)))))
        );
    }

    #[test]
    fn parse_subtract_statement() {
        let parsed = parse("12 - 34");
        assert_eq!(
            parsed,
            Ok(("", Sub(Box::new(Num(12.0)), Box::new(Num(34.0)))))
        );
    }

    #[test]
    fn parse_nested_add_sub_statements() {
        let parsed = parse("12 - 34 + 15 - 9");
        assert_eq!(
            parsed,
            Ok((
                "",
                Sub(
                    Box::new(Add(
                        Box::new(Sub(Box::new(Num(12.0)), Box::new(Num(34.0)))),
                        Box::new(Num(15.0))
                    )),
                    Box::new(Num(9.0))
                )
            ))
        );
    }

    #[test]
    fn test_parse_multi_level_expression() {
        let parsed = parse("1 * 2 + 3 / 4 ^ 6");
        let expected = Add(
            Box::new(Mul(Box::new(Num(1.0)), Box::new(Num(2.0)))),
            Box::new(Div(
                Box::new(Num(3.0)),
                Box::new(Exp(Box::new(Num(4.0)), Box::new(Num(6.0)))),
            )),
        );
        assert_eq!(parsed, Ok(("", expected)));
    }

    #[test]
    fn test_parse_expression_with_parantheses() {
        let parsed = parse("(1 + 2) * 3");
        let expected = Mul(
            Box::new(Add(Box::new(Num(1.0)), Box::new(Num(2.0)))),
            Box::new(Num(3.0)),
        );
        assert_eq!(parsed, Ok(("", expected)));
    }

    #[test]
    fn test_sin() {
        let parsed = parse("sin(4)");
        let expected = Sin(Box::new(Num(4.0)));
        assert_eq!(parsed, Ok(("", expected)));
    }

    #[test]
    fn test_decimals() {
        let parsed = parse("1.1 + 1.2");
        assert_eq!(
            parsed,
            Ok(("", Add(Box::new(Num(1.1)), Box::new(Num(1.2)))))
        );
    }
}
