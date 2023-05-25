use clap::Parser;
use evaluator::evaluate;

use crate::parser::parse;

use std::error::{Error};
use std::fs::File;
use std::io::{self, Write};
use std::io::Read;
use std::result::Result;
use std::time::{Instant};

mod evaluator;
mod parser;
mod types;

#[derive(Parser,Default,Debug)]
#[clap(version, about="A simple calculator. Written by Abhishek Sathiabalan & Balaji Sivaraman. MIT license.")]
struct Arguments {
    /// file to parse against
    #[clap(short, long)]
    file: Option<String>
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Arguments::parse();
    match args.file {
        Some(path) => {
            let mut test_file = File::open(path)?;
            let mut input_file_contents = String::new();
            test_file.read_to_string(&mut input_file_contents)?;
            for line in input_file_contents.lines().by_ref() {
                process(line);
            }
        }
        None => {
            let stdin = io::stdin(); 
            loop {
                print!(">");
                io::stdout().flush()?;
                let mut user_input = String::new();
                stdin.read_line(&mut user_input)?;
                process(&user_input);
            }
        }
    }    
    Ok(())
}

fn process(line: &str) {
    if line.is_empty() {
        return;
    }
    let start = Instant::now();
    match calculate(line) {
        Ok(result) => {
            let duration = start.elapsed();
            println!("{:?} ({:?})", result, duration)
        },
        Err(error) => eprintln!("{:?}", error)
    }
}

fn calculate(input: &str) -> Result<f64, String> {
    match parse(input) {
        Ok( expr ) => {
            let (_, parsed_line) = expr;
            let result = evaluate(parsed_line);
            Ok(result)
        }
        Err(error) => Err(error.to_string())
    }
}
