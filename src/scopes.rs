use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::Node;

pub struct Scopes {
    pub bindings: Vec<HashMap<String, Rc<RefCell<Node>>>>,
}

impl Scopes {
    pub fn new() -> Self {
        Self {
            bindings: Vec::new(),
        }
    }

    pub fn push(&mut self) {
        self.bindings.push(HashMap::new());
    }

    pub fn pop(&mut self) {
        self.bindings.pop();
    }

    pub fn declare(&mut self, name: &str, node: Rc<RefCell<Node>>) {
        self.bindings
            .last_mut()
            .unwrap()
            .insert(name.to_string(), node);
    }

    pub fn get(&self, name: &str) -> Option<Rc<RefCell<Node>>> {
        for bindings in self.bindings.iter().rev() {
            if let Some(binding) = bindings.get(name) {
                return Some(binding.clone());
            }
        }
        None
    }

    pub fn set(&mut self, name: &str, node: Rc<RefCell<Node>>) {
        for bindings in self.bindings.iter_mut().rev() {
            if let Some(binding) = bindings.get_mut(name) {
                *binding = node;
                return;
            }
        }
        panic!("variable `{}` not found", name);
    }
}
