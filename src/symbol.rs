use std::collections::HashMap;

pub struct SymbolTable {
    table: HashMap<String, i16>,
    ram_address_counter: i16
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let mut default_table = HashMap::new();
        default_table.insert("SP".to_string(), 0);
        default_table.insert("LCL".to_string(), 1);
        default_table.insert("ARG".to_string(), 2);
        default_table.insert("THIS".to_string(), 3);
        default_table.insert("THAT".to_string(), 4);
        default_table.insert("SCREEN".to_string(), 16384);
        default_table.insert("KBD".to_string(), 24567);

        for i in 0..16 {
            default_table.insert(format!("R{}", i).to_string(), i);
        }

        SymbolTable {
            table: default_table,
            ram_address_counter: 16
        }
    }

    pub fn add_entry(&mut self, symbol: String, address: i16) {
        self.table.insert(symbol, address);
    }

    pub fn add_ram_entry(&mut self, symbol: &String) -> i16 {
        self.add_entry(symbol.clone(), self.ram_address_counter);
        self.ram_address_counter += 1;

        self.ram_address_counter - 1
    }

    pub fn contains(&self, symbol: &String) -> bool {
        self.table.contains_key(symbol)
    }

    pub fn get_address(&self, symbol: &String) -> Option<&i16> {
        self.table.get(symbol)
    }
}
