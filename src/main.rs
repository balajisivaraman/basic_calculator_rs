use crate::parser::parse;

mod evaluator;
mod parser;
mod types;

fn main() {
    println!("{:?}", parse("1234"));
}
