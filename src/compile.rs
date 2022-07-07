use std::borrow::Borrow;
use crate::ast::{Expr, Module, Statement};

pub enum Ops {
    LoadConst(i32),
    LoadConstTop,
    Store,
}

pub fn compile_expr(expr: Box<Expr>) -> Vec<Ops> {
    use self::Expr::*;
    return match expr {
        Integer(n) => Vec::from([Ops::LoadConst(n)])
    };
}

pub fn compile_stmt(statement: Box<Statement>) -> Vec<Ops> {
    use self::Statement::*;
    return match statement {
        Assignment(name, expr) => {
            let mut ops = compile_expr(expr);
            ops.push(Ops::Store);
            ops
        }
    }
}

pub fn compile_body(body: Vec<Box<Statement>>) -> Vec<Ops> {
    let mut operations: Vec<Ops> = Vec::new();
    for statement in body {
        operations.extend(compile_stmt(statement));
    }
    return operations;
}

pub struct Env {
    pub co_consts: Vec<Expr>
}

pub fn compile(module: Module) -> Vec<Ops> {
    let mut env = Env {
        co_consts: Vec::new()
    };

    let mut operations: Vec<Ops> = Vec::new();
    operations.extend(compile_body(module.body));
    return operations;
}