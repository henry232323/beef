use crate::ast::{Expr, Opcode, Statement};
use crate::Module;

use std::collections::HashMap;

use std::iter::zip;

pub struct Runtime {
    module: Module,
}

impl Runtime {
    pub fn new(module: Module) -> Runtime {
        return Runtime { module };
    }

    fn eval_expr(&mut self, expr: &Box<Expr>, env: &mut HashMap<String, Box<Expr>>) -> Expr {
        use self::Expr::*;
        return match &**expr {
            BinaryOp(expr1, Opcode::Add, expr2) => {
                let operand1 = self.eval_expr(&expr1, env);
                let operand2 = self.eval_expr(&expr2, env);

                match (operand1, operand2) {
                    (Float(x), Float(y)) => Float(x + y),
                    (Float(x), Integer(y)) => Float(x + y as f32),
                    (Integer(x), Float(y)) => Float(x as f32 + y),
                    (Integer(x), Integer(y)) => Integer(x + y),
                    _ => {
                        panic!("Cannot perform binary op");
                    }
                }
            }
            UnaryOp(Opcode::Add, expr) => {
                let new_val = self.eval_expr(&expr, env);
                match new_val {
                    Float(x) => Float(x),
                    Integer(x) => Integer(x),
                    _ => panic!("Cannot +"),
                }
            }
            UnaryOp(Opcode::Sub, expr) => {
                let new_val = self.eval_expr(&expr, env);
                match new_val {
                    Float(x) => Float(-x),
                    Integer(x) => Integer(-x),
                    _ => panic!("Cannot invert"),
                }
            }
            Variable(name) => match env.get(name) {
                Option::None => {
                    panic!("Cannot resolve name!")
                }
                Some(x) => *x.clone(),
            },
            FunctionCall(expr, args) => {
                let func = self.eval_expr(&expr, env);
                match func {
                    Function(_name, arg_names, body) => {
                        if _name == "print" {
                            println!("{:#?}", args[0]);
                            return None;
                        }

                        let mut new_env = env.clone();

                        for (arg_name, arg_value) in zip((*arg_names).clone(), args) {
                            new_env.insert(arg_name, arg_value.clone());
                        }

                        match self.eval_body(&body, &mut new_env) {
                            Option::None => None,
                            Some(x) => x,
                        }
                    }
                    _ => {
                        panic!("Is not callable!");
                    }
                }
            }
            // ItemSubscription(_, _) => { None }
            // AttrAccess(_, _) => { None }
            expr => expr.clone(),
        };
    }

    fn eval_stmt(
        &mut self,
        statement: &Box<Statement>,
        env: &mut HashMap<String, Box<Expr>>,
    ) -> Option<Expr> {
        use self::Statement::*;
        return match &**statement {
            Expression(expr) => Some(self.eval_expr(&expr, env)),
            If(cond, body, elsebody) => {
                let f_cond = self.eval_expr(&cond, env);
                match f_cond {
                    Expr::Boolean(val) => {
                        if val {
                            self.eval_body(&body, env)
                        } else {
                            match elsebody {
                                Some(e_body) => self.eval_body(&e_body, env),
                                None => None,
                            }
                        }
                    }
                    _expr => panic!("Cannot if!"),
                }
            }
            Function(_, _, _) => None,
            Assignment(name, value) => {
                let mut new_env = env.clone();
                new_env.insert(name.clone(), value.clone());
                None
            }
            Return(expr) => {
                let res = self.eval_expr(&expr, env);
                Some(res)
            }
            While(_, _) => None,
            For(_, _, _) => None,
            Import(_) => None,
            Try(_, _) => None,
        };
    }

    fn eval_body(
        &mut self,
        body: &Vec<Box<Statement>>,
        env: &mut HashMap<String, Box<Expr>>,
    ) -> Option<Expr> {
        for statement in body {
            match self.eval_stmt(statement, env) {
                None => {}
                Some(x) => return Some(x),
            }
        }

        return None;
    }

    pub fn eval(&mut self) {
        let mut env: HashMap<String, Box<Expr>> = HashMap::new();
        env.insert(
            "print".to_string(),
            Box::from(Expr::Function(
                "print".to_string(),
                Box::from(Vec::from(["item".to_string()])),
                Vec::new(),
            )),
        );
        self.eval_body(&self.module.body.clone(), &mut env);
    }
}
