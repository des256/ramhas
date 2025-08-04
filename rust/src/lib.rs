use {
    graphviz_rust::dot_structures::Attribute,
    std::{any::Any, cell::RefCell, fmt::Display, rc::Rc},
};

mod visualize;
pub use visualize::*;

mod token;
pub use token::*;

mod tokenizer;
pub use tokenizer::*;

mod parser;
pub use parser::*;

mod scopes;
pub use scopes::*;

mod ty;
pub use ty::*;

mod value;
pub use value::*;

pub trait Expr: Any + Display {
    fn as_any(&mut self) -> &mut dyn Any;
    fn visualize(
        &mut self,
        gen_id: &str,
        visualizer: &mut Visualizer,
        attributes: &mut Vec<Attribute>,
    );
}

mod expr;
pub use expr::*;

pub trait Symbols {
    fn declare(&mut self, name: &str, expr: Rc<RefCell<dyn Expr>>);
    fn get(&self, name: &str) -> Option<Rc<RefCell<dyn Expr>>>;
    fn set(&mut self, name: &str, expr: Rc<RefCell<dyn Expr>>);
    fn push_scope(&mut self);
    fn pop_scope(&mut self);
}

pub trait Ctrl: Any + Display {
    fn as_any(&mut self) -> &mut dyn Any;
    fn visualize(
        &mut self,
        gen_id: &str,
        visualizer: &mut Visualizer,
        attributes: &mut Vec<Attribute>,
    );
}

mod ctrl;
pub use ctrl::*;
