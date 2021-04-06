use crate::types::Expr;
use crate::types::Expr::*;

use nom::branch::alt;
use nom::character::complete::{char, digit1, space0};
use nom::combinator::map;
use nom::multi::many1;
use nom::sequence::{delimited, tuple};
use nom::IResult;
use std::str::FromStr;

pub(crate) fn parse(input: &str) -> IResult<&str, Expr> {
    parse_basic_expr(input)
}

fn parse_basic_expr(input: &str) -> IResult<&str, Expr> {
    let (input, num1) = parse_number(input)?;
    let (input, exprs) = many1(parse_math_expr)(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_math_expr(input: &str) -> IResult<&str, (char, Expr)> {
    tuple((alt((char('+'), char('-'))), parse_number))(input)
}

fn parse_expr(expr: Expr, rem: Vec<(char, Expr)>) -> Expr {
    rem.into_iter().fold(expr, |acc, val| parse_op(val, acc))
}

fn parse_op(tup: (char, Expr), expr1: Expr) -> Expr {
    let (op, expr2) = tup;
    match op {
        '+' => EAdd(Box::new(expr1), Box::new(expr2)),
        '-' => ESub(Box::new(expr1), Box::new(expr2)),
        _ => panic!("Unknown Operation"),
    }
}

fn parse_enum(parsed_num: &str) -> Expr {
    let num = i32::from_str(parsed_num).unwrap();
    ENum(num)
}

fn parse_number(input: &str) -> IResult<&str, Expr> {
    map(delimited(space0, digit1, space0), parse_enum)(input)
}

#[cfg(test)]
mod tests {
    use crate::parser::parse;
    use crate::types::Expr::*;

    #[test]
    fn parse_add_statement() {
        let parsed = parse("12 + 34");
        assert_eq!(
            parsed,
            Ok(("", EAdd(Box::new(ENum(12)), Box::new(ENum(34)))))
        );
    }

    #[test]
    fn parse_subtract_statement() {
        let parsed = parse("12 - 34");
        assert_eq!(
            parsed,
            Ok(("", ESub(Box::new(ENum(12)), Box::new(ENum(34)))))
        );
    }

    #[test]
    fn parse_nested_add_sub_statements() {
        let parsed = parse("12 - 34 + 15 - 9");
        assert_eq!(
            parsed,
            Ok((
                "",
                ESub(
                    Box::new(EAdd(
                        Box::new(ESub(Box::new(ENum(12)), Box::new(ENum(34)))),
                        Box::new(ENum(15))
                    )),
                    Box::new(ENum(9))
                )
            ))
        );
    }
}
