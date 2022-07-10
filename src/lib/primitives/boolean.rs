use std::error::Error;
use crate::lib::primitives::object::Object;

struct Boolean{}
impl Object for Boolean {
    fn eq(&self, other: &dyn Object) -> Result<bool, dyn Error> {
        let Boolean(self_val) = self;
        return match *other {
            Expr::Boolean(other_val) => Ok(self_val == other_val),
            _ => Ok(false)
        };
    }

    fn add(&self, other: &dyn Object) -> Result<None, dyn Error> {
        return Err(());
    }
}
