use std::collections::HashMap;

pub type Environment = HashMap<String, Atom>;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Instructions {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    Not,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Atom {
    Int(i32),
    Key(String),
    Boolean(bool),
    Instructions(Instructions),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Expr {
    Constant(Atom),
    Application(Box<Expr>, Vec<Expr>),
    If(Box<Expr>, Box<Expr>),
    IfElse(Box<Expr>, Box<Expr>, Box<Expr>),
    Quote(Vec<Expr>),
}
