use crate::lib::primitives::boolean::Boolean;
use crate::lib::primitives::types::{Type, TypeEnum};

pub struct Object {}

impl Type for Object {
    fn eq(&self, other: &TypeEnum) -> Result<Box<TypeEnum>, &str> {
        match other {
            TypeEnum::Object(_) => Ok(Boolean::new(false)),
            TypeEnum::Float(float) => Ok(Boolean::new(false)),
            TypeEnum::Integer(int) => Ok(Boolean::new(false)),
            TypeEnum::Boolean(_) => Ok(Boolean::new(false)),
        }
    }

    fn add(&self, other: &TypeEnum) -> Result<Box<TypeEnum>, &str> {
        match other {
            TypeEnum::Object(_) => Err("Cannot add Object to Object"),
            TypeEnum::Float(float) => Err("Cannot add Object to Float"),
            TypeEnum::Integer(int) => Err("Cannot add Object to Integer"),
            TypeEnum::Boolean(_) => Err("Cannot add Object to Boolean"),
        }
    }
}
