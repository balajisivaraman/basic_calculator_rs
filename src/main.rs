use evaluator::evaluate;

use crate::parser::parse;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::process;
use std::result::Result;

mod evaluator;
mod parser;
mod types;

fn main() -> Result<(), Box<dyn Error>> {
    let mut test_file = File::open("test.bc")?;
    let mut input_file_contents = String::new();
    test_file.read_to_string(&mut input_file_contents)?;
    for line in input_file_contents.lines().by_ref() {
        if line.is_empty() {
            continue;
        }
        match calculate(line) {
            Ok(result) => println!("{:?}", result),
            Err(error) => eprintln!("{:?}", error)
        }

    }
    Ok(())
}

fn calculate(input: &str) -> Result<f64, Box<dyn Error>> {
    let (input, parsed_line) = parse(input).unwrap();
    println!("{:?}", parsed_line);
    if !input.is_empty() {
        eprintln!("parsing error, input remaining {:?}", input);
        process::exit(1);
    }
    let result = evaluate(parsed_line);
    return Ok(result);
}
