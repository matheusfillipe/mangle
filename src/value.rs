use std::collections::HashMap;
use std::fmt;

use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Stack(Vec<i64>),
}

#[derive(Debug)]
pub struct MangleError(pub String);

impl fmt::Display for MangleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for MangleError {}

#[derive(Debug, Default)]
pub struct Scope {
    pub variables: HashMap<String, Value>,
    pub emitted_output: bool,
}

impl Scope {
    pub fn lookup(&self, word: &str) -> Value {
        match self.variables.get(word) {
            Some(value) => value.clone(),
            None => Value::Int(grapheme_len(word) as i64),
        }
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Value {
        self.variables.insert(name.to_string(), value.clone());
        value
    }

    pub fn mark_output(&mut self) {
        self.emitted_output = true;
    }
}

pub fn grapheme_len(text: &str) -> usize {
    text.graphemes(true).count()
}

pub fn as_int(value: &Value) -> i64 {
    match value {
        Value::Int(n) => *n,
        Value::Stack(items) => items.len() as i64,
    }
}

pub fn render(value: &Value) -> String {
    match value {
        Value::Int(n) => n.to_string(),
        Value::Stack(items) => items
            .iter()
            .map(i64::to_string)
            .collect::<Vec<_>>()
            .join(" "),
    }
}
