#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate structopt;

mod generator;
mod helper;
mod instruction;
mod parser;
mod symbol;

use std::path::PathBuf;
use structopt::StructOpt;

use crate::generator::Generator;
use crate::parser::parse_file;

#[derive(StructOpt)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf
}

fn main() {
    let opt = Opt::from_args();
    let file_path = opt.file.as_path();
    let file_stem = file_path.file_stem().unwrap();
    let instructions = parse_file(&file_path);
    let mut generator = Generator::new(instructions);

    generator.generate_binary_code();
    generator.generate_binary_file(file_stem.to_str().unwrap());
}
