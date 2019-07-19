use std::fs::File;
use std::io::Write;

use crate::instruction::InstructionType;
use crate::symbol::SymbolTable;

pub struct Generator {
    instructions: Vec<InstructionType>,
    symbol_table: SymbolTable,
    binary_instructions: Option<Vec<String>>,
}

impl Generator {
    pub fn new(instructions: Vec<InstructionType>) -> Generator {
        let symbol_table = SymbolTable::new();
        Generator {
            instructions,
            symbol_table,
            binary_instructions: None,
        }
    }

    pub fn generate_binary_code(&mut self) {
        self.symbol_table_pass();
        self.instruction_to_binary_pass();
    }

    pub fn generate_binary_file(&self, file_name: &str) {
        if let Some(binary_instructions) = &self.binary_instructions {
            let file_name = format!("{}.hack", file_name);
            let mut binary_file = File::create(file_name).expect("Could not create file");

            for instruction in binary_instructions.iter() {
                writeln!(binary_file, "{}", instruction.clone()).unwrap();
            }
        } else {
            panic!("No binary instructions available to write to file")
        }
    }

    fn symbol_table_pass(&mut self) {
        let mut rom_count = 0;

        for instruction in self.instructions.as_slice() {
            match instruction {
                InstructionType::L(instr) => {
                    self.symbol_table
                        .add_entry(instr.symbol.clone(), rom_count);
                }
                InstructionType::C(_) | InstructionType::A(_) => {
                    rom_count += 1;
                }
            }
        }
    }

    fn instruction_to_binary_pass(&mut self) {
        let symbol_table = &mut self.symbol_table;

        let binary_instructions: Vec<String> = self
            .instructions
            .iter_mut()
            .filter_map(|instruction| {
                match instruction {
                    InstructionType::C(instr) => Some(instr.to_binary()),
                    InstructionType::A(instr) => {
                        if !instr.resolved_symbol_to_address() {
                            let symbol = instr.symbol.clone().unwrap();
                            instr.address = if symbol_table.contains(&symbol) {
                                Some(*symbol_table.get_address(&symbol).unwrap())
                            } else {
                                Some(symbol_table.add_ram_entry(&symbol))
                            }
                        };
                        Some(instr.to_binary())
                    }
                    InstructionType::L(_) => None,
                }
            })
            .collect();
        self.binary_instructions = Some(binary_instructions);
    }
}
