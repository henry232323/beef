use std::any::Any;
use std::error::Error;
use crate::ast::{Expr, Statement};
use crate::Module;

trait Object {
    fn eq(&self, other: &dyn Object) -> Result<bool, dyn Error>;
    fn add(&self, other: &dyn Object) -> Result<dyn Any, dyn Error>;
}

struct Boolean{}
impl Object for Expr::Boolean {
    fn eq(&self, other: &dyn Object) -> Result<bool, dyn Error> {
        let Expr::Boolean(self_val) = self;
        return match *other {
            Expr::Boolean(other_val) => Ok(self_val == other_val),
            _ => Ok(false)
        };
    }

    fn add(&self, other: &dyn Object) -> Result<None, dyn Error> {
        return Err(());
    }
}

impl Object for Expr::Integer {
    fn eq(&self, other: &dyn Object) -> Result<bool, dyn Error> {
        let Expr::Integer(self_val) = self;
        return match *other {
            Expr::Integer(other_val) => Ok(self_val == other_val),
            _ => false
        };
    }

    fn add(&self, other: &dyn Object) -> Result<i32, dyn Error> {
        let Expr::Integer(self_val) = self;
        return match *other {
            Expr::Integer(other_val) => Ok(self_val + other_val),
            _ => Err(false)
        };
    }
}

impl Object for Expr::Float {
    fn eq(&self, other: &dyn Object) -> Result<bool, dyn Error> {
        let Expr::Float(self_val) = self;
        return match *other {
            Expr::Float(other_val) => Ok(self_val == other_val),
            _ => false
        };
    }

    fn add(&self, other: &dyn Object) -> Result<f32, dyn Error> {
        let Expr::Float(self_val) = self;
        return match *other {
            Expr::Float(other_val) => Ok(self_val + other_val),
            _ => Err(false)
        };
    }
}

pub trait Function {}

pub struct Runtime {
    module: Module,
}

impl Runtime {
    pub fn new(module: Module) -> Runtime {
        return Runtime {
            module
        };
    }

    fn eval_expr(&mut self, expr: Box<Expr>) {
        use self::Expr::*;
        match *expr {
            Boolean(_) => {}
            Integer(_) => {}
            Float(_) => {}
            BinaryOp(_, _, _) => {}
            UnaryOp(_, _) => {}
            Error => {}
            Str(_) => {}
            List(_) => {}
            Variable(_) => {}
            FunctionCall(_, _) => {}
            ItemSubscription(_, _) => {}
            AttrAccess(_, _) => {}
        }
    }

    fn eval_stmt(&mut self, statement: Box<Statement>) {
        use self::Statement::*;
        match *statement {
            Expression(_) => {}
            If(_, _, _) => {}
            Function(_, _, _) => {}
            Assignment(_, _) => {}
            While(_, _) => {}
        }
    }

    fn eval_body(&mut self, body: &Vec<Box<Statement>>) {
        for statement in body {
            eval_stmt(&statement)
        }
    }

    pub fn eval(&mut self) {
        self.eval_body(&self.module.body)
    }
}