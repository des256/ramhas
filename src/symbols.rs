use {crate::*, std::collections::HashMap};

#[derive(Debug, Clone)]
pub struct Symbols {
    symbolses: Vec<HashMap<String, Id<Expr>>>,
}

impl Symbols {
    pub fn new() -> Self {
        Self {
            symbolses: Vec::new(),
        }
    }

    pub fn push_scope(&mut self) {
        self.symbolses.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.symbolses.pop();
    }

    pub fn declare(&mut self, name: &str, expr_id: Id<Expr>) {
        self.symbolses
            .last_mut()
            .unwrap()
            .insert(name.to_string(), expr_id);
    }

    pub fn get(&self, name: &str) -> Option<Id<Expr>> {
        for symbols in self.symbolses.iter().rev() {
            if let Some(expr_id) = symbols.get(name) {
                return Some(*expr_id);
            }
        }
        None
    }

    pub fn set(&mut self, name: &str, expr_id: Id<Expr>) {
        for symbols in self.symbolses.iter_mut().rev() {
            if let Some(symbol) = symbols.get_mut(name) {
                *symbol = expr_id;
                return;
            }
        }
    }
}
