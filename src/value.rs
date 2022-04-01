pub enum Type {
    Int,
    String,
    Stack,
    Null,
}

pub struct Value {
    pub type_: Type,
    pub value: String,
}

impl Value {
    pub fn new(value: String, type_: Type) -> Value {
        Value { type_, value }
    }

    pub fn empty() -> Value {
        Value {
            value: String::new(),
            type_: Type::Null,
        }
    }
}
