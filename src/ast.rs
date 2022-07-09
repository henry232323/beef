// pub enum Expr {
//     Num(i32),
//     BinaryOp(Box<Expr>, Opcode, Box<Expr>),
//     Error,
//     Str(String)
// }

use std::fmt::{Debug, Error, Formatter};

pub enum Expr {
    Boolean(bool),
    Integer(i32),
    Float(f32),
    BinaryOp(Box<Expr>, Opcode, Box<Expr>),
    UnaryOp(Opcode, Box<Expr>),
    Error,
    Str(String),
    List(Vec<Box<Expr>>),
    Variable(String),
    FunctionCall(Box<Expr>, Vec<Box<Expr>>),
    ItemSubscription(Box<Expr>, Box<Expr>),
    AttrAccess(Box<Expr>, String),
}

pub enum Statement {
    Expression(Box<Expr>),
    If(Box<Expr>, Vec<Box<Statement>>, Option<Vec<Box<Statement>>>),
    Function(String, Box<Vec<String>>, Vec<Box<Statement>>),
    Assignment(String, Box<Expr>),
    While(Box<Expr>, Vec<Box<Statement>>),
    For (String, Box<Expr>, Vec<Box<Statement>>),
}


#[derive(Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
    Pow
}

pub struct Module {
    pub body: Vec<Box<Statement>>
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match &*self {
            Integer(n) => write!(fmt, "Integer({:#?})", n),
            Float(n) => write!(fmt, "Float({:#?})", n),
            Boolean(b) => write!(fmt, "Boolean({:#?})", b),
            BinaryOp(ref l, op, ref r) => write!(fmt, "BinaryOp({:#?}, {:#?}, {:#?})", l, op, r),
            UnaryOp(op, ref r) => write!(fmt, "UnaryOp({:#?}, {:#?})", op, r),
            Str(s) => write!(fmt, "Str({:#?})", s),
            List(l) => write!(fmt, "List({:#?})", l),
            Variable(v) => write!(fmt, "Variable({:#})", v),
            FunctionCall(ref fun, args) => write!(fmt, "FunctionCall({:#?}, args={:#?})", fun, args),
            ItemSubscription(ref expr, ref index) => write!(fmt, "ItemSubscription({:#?}, index={:#?})", expr, index),
            AttrAccess(ref expr, attr) => write!(fmt, "AttrAccess({:#?}, attr={:#?})", expr, attr),
            Error => write!(fmt, "error"),
        }
    }
}

impl Debug for Statement {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Statement::*;
        match &*self {
            Expression(ref expr) => write!(fmt, "Expression({:#?})", expr),
            If(ref cond, body, elsebody) => write!(fmt, "If(cond=({:#?}), body={:#?}, elsebody={:#?})", cond, body, elsebody),
            Function(ref name, args, body) => write!(fmt, "Function(name=({:#?}), args={:#?}, body={:#?})", name, args, body),
            Assignment(ref name, value) => write!(fmt, "Assignment(name=({:#?}), value={:#?})", name, value),
            While(ref cond, body) => write!(fmt, "While(cond=({:#?}), body={:#?})", cond, body),
            For(ref name, value, body) => write!(fmt, "For(name=({:#?}), value={:#?}, body={:#?})", name, value, body),
        }
    }
}

impl Debug for Opcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Opcode::*;
        match *self {
            Mul => write!(fmt, "Op::Mul"),
            Div => write!(fmt, "Op::Div"),
            Add => write!(fmt, "Op::Add"),
            Sub => write!(fmt, "Op::Sub"),
            Pow => write!(fmt, "Op::Pow"),
        }
    }
}