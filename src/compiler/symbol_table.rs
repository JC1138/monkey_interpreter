use std::{cell::{Cell, RefCell}, collections::HashMap};

#[derive(Debug, Clone)]
pub struct SymbolScope(String);

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub scope: SymbolScope,
    pub idx: u16,
}

impl Symbol {
    pub fn new(name: &str, scope: &SymbolScope, idx: u16) -> Self {
        Self {
            name: name.to_string(),
            scope: scope.clone(),
            idx,
        }
    }
}

#[derive(Debug)]
pub struct SymbolTable {
    store: RefCell<HashMap<String, Symbol>>,
    num_defs: Cell<u16>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            store: RefCell::new(HashMap::new()),
            num_defs: Cell::new(0),
        }
    }

    pub fn define(&self, name: &str) -> u16 {
        let mut store = self.store.borrow_mut();
        let num_defs = self.num_defs.get();
        store.insert(name.to_string(), Symbol::new(name, &SymbolScope("Global".to_string()), num_defs));
        self.num_defs.set(num_defs + 1);

        num_defs
    }

    pub fn resolve(&self, name: &str) -> Option<u16> {
        Some(self.store.borrow().get(name)?.idx)
    }
}
