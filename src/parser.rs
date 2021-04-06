use crate::types::Expr;
use crate::types::Expr::*;
use nom::sequence::tuple;

use nom::branch::alt;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0},
    combinator::map,
    sequence::delimited,
    IResult,
};
use std::str::FromStr;

pub(crate) fn parse(input: &str) -> IResult<&str, Expr> {
    let (input, (num1, op, num2)) =
        tuple((parse_number, alt((tag("+"), tag("-"))), parse_number))(input)?;
    if op == "+" {
        Ok((input, EAdd(Box::new(num1), Box::new(num2))))
    } else {
        Ok((input, ESub(Box::new(num1), Box::new(num2))))
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
}
