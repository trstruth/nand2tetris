use std::collections::HashMap;

pub struct SymbolTable {
    pub s: HashMap<String, u32>,
    variable_offset_counter: u32,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut table: HashMap<String, u32> = HashMap::new();
        for idx in 0..16 {
            table.insert(format!("R{}", idx), idx);
        }

        for (idx, symbol) in vec!["SP", "LCL", "ARG", "THIS", "THAT"].iter().enumerate() {
            table.insert(symbol.to_string(), idx as u32);
        }

        table.insert("SCREEN".to_owned(), 16384);
        table.insert("KBD".to_owned(), 24576);

        SymbolTable {
            s: table,
            variable_offset_counter: 16,
        }
    }

    pub fn add_symbol(&mut self, symbol: &str, mem_location: Option<u32>) -> u32 {
        if let Some(mem_idx) = mem_location {
            self.s.insert(symbol.to_owned(), mem_idx);
            return mem_idx;
        }
        self.s
            .insert(symbol.to_owned(), self.variable_offset_counter);
        self.variable_offset_counter += 1;
        return self.variable_offset_counter - 1;
    }

    pub fn get_symbol_address(&self, symbol: &str) -> Option<u32> {
        self.s.get(symbol).map(|v| v.clone())
    }

    pub fn contains(&self, symbol: &str) -> bool {
        match self.get_symbol_address(symbol) {
            Some(_) => true,
            None => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_new_variable() {
        let mut st = SymbolTable::new();
        let mem_location = st.add_symbol("counter", None);

        assert_eq!(mem_location, 16);

        assert_eq!(
            true,
            match st.get_symbol_address("counter") {
                Some(16) => true,
                _ => false,
            }
        );
    }
}
