use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

use regex::Regex;

use crate::instruction::{AInstruction, CInstruction, InstructionType, LInstruction};

#[derive(Debug)]
enum ParseError {
    InvalidCharacter,
}

pub fn parse_file(file_path: &Path) -> Vec<InstructionType> {
    let file = File::open(file_path).expect("Could not open file");
    let buffer = io::BufReader::new(file);
    let lines = buffer.lines();

    parse_lines(lines)
}

fn parse_lines(lines: io::Lines<io::BufReader<File>>) -> Vec<InstructionType> {
    println!("Parsing lines");
    lines
        .filter_map(|line| {
            let unwrap_line = line.expect("Could not unwrap line");
            parse_line(unwrap_line).expect("invalid line")
        })
        .collect()
}

fn parse_line(line: String) -> Result<Option<InstructionType>, ParseError> {
    if line.is_empty() {
        return Ok(None);
    }
    lazy_static! {
        static ref LINE_CLEANUP: Regex = Regex::new(r"(.+)(//.*)").unwrap();
    }

    let maybe_captures = LINE_CLEANUP.captures(&line);
    let mut cleaned_line = line.clone();
    if let Some(captures) = maybe_captures {
        cleaned_line = captures.get(1).unwrap().as_str().to_string();
    }
    cleaned_line = cleaned_line.trim().to_string();
    let mut chars = cleaned_line.chars();

    match chars.next().unwrap() {
        '@' => Ok(Some(InstructionType::A(AInstruction::new(cleaned_line)))),
        '(' => Ok(Some(InstructionType::L(LInstruction::new(cleaned_line)))),
        '0' | '1' | '-' | '!' | 'A' | 'D' | 'M' => {
            Ok(Some(InstructionType::C(CInstruction::new(cleaned_line))))
        }
        ' ' | '/' => {Ok(None)},
        _ => Err(ParseError::InvalidCharacter),
    }
}
