use std::collections::HashMap;

use crate::operators::{get_operator, Flow, Operator};
use crate::value::{grapheme_len, render, MangleError, Scope, Value};

const SENTENCE_ENDINGS: &[char] = &['.', ',', ';', ':', '?', '!'];
pub const MAX_OPS: usize = 1_000_000;

struct Op {
    operator: Operator,
    args: Vec<String>,
}

struct Sentence {
    #[allow(dead_code)]
    label: Option<String>,
    ops: Vec<Op>,
}

pub struct Interpreter {
    word_separator: char,
}

impl Interpreter {
    pub fn new(word_separator: char) -> Self {
        Self { word_separator }
    }

    pub fn eval(&mut self, source: &str) -> Result<String, MangleError> {
        let mut scope = Scope::default();
        let last = execute(source, self.word_separator, &mut scope)?;
        if scope.emitted_output {
            Ok(String::new())
        } else {
            Ok(render(&last))
        }
    }
}

pub fn execute(text: &str, separator: char, scope: &mut Scope) -> Result<Value, MangleError> {
    execute_with_limit(text, separator, scope, MAX_OPS)
}

pub fn execute_with_limit(
    text: &str,
    separator: char,
    scope: &mut Scope,
    max_ops: usize,
) -> Result<Value, MangleError> {
    let (sentences, labels) = parse_program(text, separator)?;
    scope.assign("it", Value::Int(0));
    let mut last_value = Value::Int(0);
    let mut instruction_pointer = 0usize;
    let mut ops_executed = 0usize;
    while instruction_pointer < sentences.len() {
        let mut next_pointer = instruction_pointer + 1;
        for op in &sentences[instruction_pointer].ops {
            ops_executed += 1;
            if ops_executed > max_ops {
                return Err(MangleError(format!(
                    "operation limit ({}) exceeded; likely infinite loop",
                    max_ops
                )));
            }
            let args: Vec<&str> = op.args.iter().map(String::as_str).collect();
            match (op.operator.apply)(&args, scope)? {
                Flow::Next(value) => {
                    last_value = value.clone();
                    scope.assign("it", value);
                }
                Flow::Jump(label) => {
                    next_pointer = *labels.get(&label).ok_or_else(|| {
                        MangleError(format!("unknown label: {:?}", label))
                    })?;
                    break;
                }
            }
        }
        instruction_pointer = next_pointer;
    }
    Ok(last_value)
}

fn parse_program(
    text: &str,
    separator: char,
) -> Result<(Vec<Sentence>, HashMap<String, usize>), MangleError> {
    let normalized: String = text
        .chars()
        .map(|c| if matches!(c, '\n' | '\r' | '\t') { separator } else { c })
        .collect();
    let mut sentences = Vec::new();
    let mut labels: HashMap<String, usize> = HashMap::new();
    for raw in normalized.split(|ch| SENTENCE_ENDINGS.contains(&ch)) {
        let words: Vec<&str> = raw
            .split(separator)
            .map(str::trim)
            .filter(|word| !word.is_empty())
            .collect();
        if words.is_empty() {
            continue;
        }
        let (body, label) = if sentences.is_empty() {
            (words, None)
        } else {
            let split = words.len() - 1;
            (words[..split].to_vec(), Some(words[split].to_string()))
        };
        let position = sentences.len();
        if let Some(ref label) = label {
            labels.entry(label.clone()).or_insert(position);
        }
        sentences.push(Sentence {
            label,
            ops: parse_ops(&body)?,
        });
    }
    Ok((sentences, labels))
}

fn parse_ops(words: &[&str]) -> Result<Vec<Op>, MangleError> {
    let mut ops = Vec::new();
    let mut cursor = 0usize;
    while cursor < words.len() {
        let word = words[cursor];
        let operator = get_operator(word).ok_or_else(|| {
            MangleError(format!(
                "no operator for word length {}: {:?}",
                grapheme_len(word),
                word
            ))
        })?;
        let end = if operator.variadic {
            words.len()
        } else {
            cursor + 1 + operator.arity
        };
        if !operator.variadic && end > words.len() {
            return Err(MangleError(format!(
                "operator {:?} needs {} argument(s)",
                word, operator.arity
            )));
        }
        let args: Vec<String> = words[cursor + 1..end]
            .iter()
            .map(|word| word.to_string())
            .collect();
        ops.push(Op { operator, args });
        cursor = end;
    }
    Ok(ops)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(text: &str) -> i64 {
        let mut scope = Scope::default();
        match execute(text, ' ', &mut scope) {
            Ok(Value::Int(n)) => n,
            Ok(Value::Stack(items)) => items.len() as i64,
            Err(error) => panic!("expected success, got: {}", error),
        }
    }

    fn run_err(text: &str) -> String {
        let mut scope = Scope::default();
        match execute(text, ' ', &mut scope) {
            Ok(_) => panic!("expected error, got success"),
            Err(error) => error.0,
        }
    }

    #[test]
    fn sum_two_literals() {
        assert_eq!(run("add cat fat"), 6);
    }

    #[test]
    fn subtract_two_literals() {
        assert_eq!(run("reduces fat cat"), 0);
    }

    #[test]
    fn multiply_two_literals() {
        assert_eq!(run("multiply fat fat"), 9);
    }

    #[test]
    fn divide_two_literals() {
        assert_eq!(run("dividedby things cat"), 2);
    }

    #[test]
    fn modulo_two_literals() {
        assert_eq!(run("remainderops hello cat"), 2);
    }

    #[test]
    fn assign_then_sum() {
        assert_eq!(run("is x cat add x fat"), 6);
    }

    #[test]
    fn literal_passthrough() {
        assert_eq!(run("a cat"), 3);
    }

    #[test]
    fn result_register_it() {
        assert_eq!(run("reduces cat cat add it fat"), 3);
    }

    #[test]
    fn goto_skips_fallthrough() {
        let program = "jumps skip. add cat cat mid. add it fat skip";
        assert_eq!(run(program), 3);
    }

    #[test]
    fn ifzero_taken_jumps() {
        let program = "reduces cat cat when it skip. add cat cat mid. add it fat skip";
        assert_eq!(run(program), 3);
    }

    #[test]
    fn ifzero_not_taken_falls_through() {
        let program = "when cat skip. add cat cat mid. add it fat skip";
        assert_eq!(run(program), 9);
    }

    #[test]
    fn push_then_pop() {
        assert_eq!(run("pushesvalue cat mystack popreturns mystack"), 3);
    }

    #[test]
    fn graphemes_not_codepoints() {
        assert_eq!(run("add a\u{0301} cat"), 4);
    }

    #[test]
    fn unknown_label_errors() {
        assert!(run_err("jumps nowhere").contains("unknown label"));
    }

    #[test]
    fn division_by_zero_errors() {
        assert!(run_err("reduces cat cat dividedby cat it").contains("division by zero"));
    }

    #[test]
    fn unknown_operator_length_errors() {
        assert!(run_err("abcdefghijklmno cat").contains("no operator"));
    }

    #[test]
    fn arity_mismatch_errors() {
        assert!(run_err("add cat").contains("argument"));
    }

    #[test]
    fn infinite_goto_hits_op_limit() {
        let mut scope = Scope::default();
        let program = "add cat fat. jumps loop loop";
        let result = execute_with_limit(program, ' ', &mut scope, 100);
        assert!(result.unwrap_err().0.contains("operation limit"));
    }

    #[test]
    fn variable_copy_via_assign() {
        assert_eq!(run("is x cat is y x add y fat"), 6);
    }

    #[test]
    fn falls_through_to_next_sentence() {
        assert_eq!(run("add cat fat. reduces fat fat end"), 0);
    }

    #[test]
    fn pop_empty_stack_yields_zero() {
        assert_eq!(run("popreturns s"), 0);
    }

    #[test]
    fn arithmetic_coerces_stack_to_its_length() {
        assert_eq!(run("pushesvalue cat s pushesvalue fat s add s cat"), 5);
    }

    #[test]
    fn eval_renders_stack() {
        let mut interpreter = Interpreter::new(' ');
        assert_eq!(
            interpreter.eval("pushesvalue cat s pushesvalue fat s").unwrap(),
            "3 3"
        );
    }

    #[test]
    fn eval_suppresses_echo_after_output() {
        let mut interpreter = Interpreter::new(' ');
        assert_eq!(interpreter.eval("output things").unwrap(), "");
    }

    #[test]
    fn multiline_program_is_one_program() {
        assert_eq!(run("is x cat\nadd x fat"), 6);
    }

    #[test]
    fn empty_program_returns_zero() {
        assert_eq!(run(""), 0);
    }

    #[test]
    fn custom_separator() {
        let mut scope = Scope::default();
        match execute("add-cat-fat", '-', &mut scope) {
            Ok(Value::Int(n)) => assert_eq!(n, 6),
            other => panic!("expected 6, got {:?}", other),
        }
    }
}
