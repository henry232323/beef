use crate::library::primitives::types::{Type, TypeEnum};

pub struct Boolean {
    pub value: bool,
}

impl Boolean {
    pub(crate) fn new(value: bool) -> Box<TypeEnum> {
        return Box::new(TypeEnum::Boolean(Boolean { value }));
    }
}

impl Type for Boolean {
    fn eq(&self, other: &TypeEnum) -> Result<Box<TypeEnum>, &str> {
        match other {
            TypeEnum::Object(_) => Ok(Boolean::new(false)),
            TypeEnum::Float(float) => Ok(Boolean::new(false)),
            TypeEnum::Integer(int) => Ok(Boolean::new(false)),
            TypeEnum::Boolean(bool) => Ok(Boolean::new(bool.value == bool.value)),
        }
    }

    fn add(&self, _: &TypeEnum) -> Result<Box<TypeEnum>, &str> {
        return Err("Cannot add boolean");
    }
}
