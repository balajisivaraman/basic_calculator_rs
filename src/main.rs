use crate::parser::parse;

mod parser;
mod types;

fn main() {
    println!("{:?}", parse("1234"));
}
