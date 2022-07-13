use crate::library::primitives::boolean::Boolean;
use crate::library::primitives::float::Float;
use crate::library::primitives::integer::Integer;
use crate::library::primitives::object::Object;

pub enum TypeEnum {
    Object(Object),
    Float(Float),
    Integer(Integer),
    Boolean(Boolean),
}

pub trait Type {
    fn eq(&self, other: &TypeEnum) -> Result<Box<TypeEnum>, &str>;
    fn add(&self, other: &TypeEnum) -> Result<Box<TypeEnum>, &str>;
}
