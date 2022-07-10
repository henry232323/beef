use std::any::Any;
use std::error::Error;

pub trait Object {
    fn eq(&self, other: &dyn Object) -> Result<bool, dyn Error>;
    fn add(&self, other: &dyn Object) -> Result<dyn Any, dyn Error>;
}