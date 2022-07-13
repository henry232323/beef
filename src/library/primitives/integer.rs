use crate::library::primitives::boolean::Boolean;
use crate::library::primitives::types::{Type, TypeEnum};

pub struct Integer {
    pub value: i32,
}

impl Integer {
    pub fn new(value: i32) -> Box<TypeEnum> {
        return Box::new(TypeEnum::Integer(Integer { value }));
    }
}

impl Type for Integer {
    fn eq(&self, other: &TypeEnum) -> Result<Box<TypeEnum>, &str> {
        match other {
            TypeEnum::Object(_) => Ok(Boolean::new(false)),
            TypeEnum::Float(float) => Ok(Boolean::new(float.value == self.value as f32)),
            TypeEnum::Integer(int) => Ok(Boolean::new(int.value == self.value)),
            TypeEnum::Boolean(_) => Ok(Boolean::new(false)),
        }
    }

    fn add(&self, other: &TypeEnum) -> Result<Box<TypeEnum>, &str> {
        match other {
            TypeEnum::Object(_) => Err("Cannot add object to int"),
            TypeEnum::Float(float) => Ok(Integer::new(float.value as i32 + self.value)),
            TypeEnum::Integer(int) => Ok(Integer::new(int.value + self.value)),
            TypeEnum::Boolean(_) => Err("Cannot add bool to int"),
        }
    }
}
