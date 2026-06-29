use crate::value::{as_int, grapheme_len, MangleError, Scope, Value};

#[derive(Debug)]
pub enum Flow {
    Next(Value),
    Jump(String),
}

type OpApply = fn(&[&str], &mut Scope) -> Result<Flow, MangleError>;

pub struct Operator {
    pub arity: usize,
    pub variadic: bool,
    pub apply: OpApply,
}

pub fn get_operator(word: &str) -> Option<Operator> {
    Some(match grapheme_len(word) {
        1 => Operator { arity: 1, variadic: false, apply: lit },
        2 => Operator { arity: 2, variadic: false, apply: assign },
        3 => Operator { arity: 2, variadic: false, apply: arith_add },
        4 => Operator { arity: 2, variadic: false, apply: ifzero },
        5 => Operator { arity: 1, variadic: false, apply: goto },
        6 => Operator { arity: 1, variadic: false, apply: print },
        7 => Operator { arity: 2, variadic: false, apply: arith_sub },
        8 => Operator { arity: 2, variadic: false, apply: arith_mul },
        9 => Operator { arity: 2, variadic: false, apply: divide },
        10 => Operator { arity: 1, variadic: false, apply: pop },
        11 => Operator { arity: 2, variadic: false, apply: push },
        12 => Operator { arity: 2, variadic: false, apply: modulo },
        13 => Operator { arity: 1, variadic: false, apply: puttext },
        14 => Operator { arity: 0, variadic: true, apply: phrase },
        _ => return None,
    })
}

type ArithFn = fn(i64, i64) -> i64;

fn arith(args: &[&str], scope: &mut Scope, op: ArithFn) -> Result<Flow, MangleError> {
    let left = as_int(&scope.lookup(args[0]));
    let right = as_int(&scope.lookup(args[1]));
    Ok(Flow::Next(Value::Int(op(left, right))))
}

fn arith_add(args: &[&str], scope: &mut Scope) -> Result<Flow, MangleError> {
    arith(args, scope, |a, b| a + b)
}

fn arith_sub(args: &[&str], scope: &mut Scope) -> Result<Flow, MangleError> {
    arith(args, scope, |a, b| a - b)
}

fn arith_mul(args: &[&str], scope: &mut Scope) -> Result<Flow, MangleError> {
    arith(args, scope, |a, b| a * b)
}

fn divide(args: &[&str], scope: &mut Scope) -> Result<Flow, MangleError> {
    let left = as_int(&scope.lookup(args[0]));
    let right = as_int(&scope.lookup(args[1]));
    if right == 0 {
        return Err(MangleError("division by zero".to_string()));
    }
    Ok(Flow::Next(Value::Int(left / right)))
}

fn lit(args: &[&str], scope: &mut Scope) -> Result<Flow, MangleError> {
    Ok(Flow::Next(scope.lookup(args[0])))
}

fn assign(args: &[&str], scope: &mut Scope) -> Result<Flow, MangleError> {
    let value = scope.lookup(args[1]);
    Ok(Flow::Next(scope.assign(args[0], value)))
}

fn print(args: &[&str], scope: &mut Scope) -> Result<Flow, MangleError> {
    let value = scope.lookup(args[0]);
    println!("{}", as_int(&value));
    scope.mark_output();
    Ok(Flow::Next(value))
}

fn goto(args: &[&str], _scope: &mut Scope) -> Result<Flow, MangleError> {
    Ok(Flow::Jump(args[0].to_string()))
}

fn ifzero(args: &[&str], scope: &mut Scope) -> Result<Flow, MangleError> {
    if as_int(&scope.lookup(args[0])) == 0 {
        return Ok(Flow::Jump(args[1].to_string()));
    }
    Ok(Flow::Next(Value::Int(0)))
}

fn push(args: &[&str], scope: &mut Scope) -> Result<Flow, MangleError> {
    let name = args[1];
    let value = as_int(&scope.lookup(args[0]));
    let mut items = match scope.variables.remove(name) {
        Some(Value::Stack(items)) => items,
        _ => Vec::new(),
    };
    items.push(value);
    scope
        .variables
        .insert(name.to_string(), Value::Stack(items.clone()));
    Ok(Flow::Next(Value::Stack(items)))
}

fn pop(args: &[&str], scope: &mut Scope) -> Result<Flow, MangleError> {
    let value = match scope.variables.get_mut(args[0]) {
        Some(Value::Stack(items)) if !items.is_empty() => items.pop().unwrap_or(0),
        _ => 0,
    };
    Ok(Flow::Next(Value::Int(value)))
}

fn puttext(args: &[&str], scope: &mut Scope) -> Result<Flow, MangleError> {
    println!("{}", args[0]);
    scope.mark_output();
    Ok(Flow::Next(Value::Int(grapheme_len(args[0]) as i64)))
}

fn phrase(args: &[&str], scope: &mut Scope) -> Result<Flow, MangleError> {
    println!("{}", args.join(" "));
    scope.mark_output();
    Ok(Flow::Next(Value::Int(
        args.iter().map(|w| grapheme_len(w) as i64).sum(),
    )))
}

fn modulo(args: &[&str], scope: &mut Scope) -> Result<Flow, MangleError> {
    let left = as_int(&scope.lookup(args[0]));
    let right = as_int(&scope.lookup(args[1]));
    if right == 0 {
        return Err(MangleError("modulo by zero".to_string()));
    }
    Ok(Flow::Next(Value::Int(left % right)))
}
