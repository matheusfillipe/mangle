use unicode_segmentation::UnicodeSegmentation;
use crate::value::{Value, Type};

pub struct OpResult {
    pub result: Result<Value, String>,
    pub assignment: bool,
    pub assign_to: String,
    pub assign_value: Value,
}

pub struct Operator {
    pub name: String,
    pub num_args: usize,
    pub func: fn(Vec<String>) -> OpResult,
}

fn strlen(str: &str) -> usize {
    str.graphemes(true).count()
}

pub fn get_operator(op: &str) -> Option<Operator> {
    match strlen(op) {
        3 => Some(Operator {
            name: "SUM".to_string(),
            num_args: 2,
            func: |args| {
                let sum = strlen(&args[0]) + strlen(&args[1]);
                OpResult {
                    result: Ok(Value::new(sum.to_string(), Type::Int)),
                    assignment: false,
                    assign_to: String::new(),
                    assign_value: Value::empty(),
                }
            },
        }),
        _ => None,
    }
}
