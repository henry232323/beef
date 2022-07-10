use crate::lib::primitives::boolean::Boolean;
use crate::lib::primitives::types::{Type, TypeEnum};

pub struct Float {
    pub(crate) value: f32,
}

impl Float {
    pub fn new(value: f32) -> Box<TypeEnum> {
        return Box::new(TypeEnum::Float(Float { value }));
    }
}

impl Type for Float {
    fn eq(&self, other: &TypeEnum) -> Result<Box<TypeEnum>, &str> {
        match other {
            TypeEnum::Object(_) => Ok(Boolean::new(false)),
            TypeEnum::Float(float) => Ok(Boolean::new(float.value == self.value)),
            TypeEnum::Integer(int) => Ok(Boolean::new(int.value as f32 == self.value)),
            TypeEnum::Boolean(_) => Ok(Boolean::new(false)),
        }
    }

    fn add(&self, other: &TypeEnum) -> Result<Box<TypeEnum>, &str> {
        match other {
            TypeEnum::Object(_) => Err("Cannot add object to int"),
            TypeEnum::Float(float) => Ok(Float::new(float.value + self.value)),
            TypeEnum::Integer(int) => Ok(Float::new(self.value + int.value as f32)),
            TypeEnum::Boolean(_) => Err("Cannot add bool to int"),
        }
    }
}
