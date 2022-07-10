use crate::lib::primitives::boolean::Boolean;
use crate::lib::primitives::float::Float;
use crate::lib::primitives::integer::Integer;
use crate::lib::primitives::object::Object;

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
