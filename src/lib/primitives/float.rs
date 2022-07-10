use std::error::Error;
use crate::lib::primitives::object::Object;

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