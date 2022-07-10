use std::error::Error;
use crate::lib::primitives::object::Object;

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