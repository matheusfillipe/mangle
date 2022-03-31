use rustyline::error::ReadlineError;
use rustyline::Editor;

extern crate dirs;
use std::fs;
use unicode_segmentation::UnicodeSegmentation;

// TODO write tests
// TODO python lib port

fn main() {
    let cache_dir = make_cache().unwrap();
    let context = Context { cache_dir };
    repl(context);
}

enum Type {
    Int,
    String,
    Stack,
    Null
}

struct Value {
    type_: Type,
    value: String,
}

impl Value {
    fn new(value: String, type_: Type) -> Value {
        Value { type_, value }
    }

    fn empty() -> Value {
        Value {
            value: String::new(),
            type_: Type::Null
        }
    }
}

struct OpResult {
    result: Result<Value, String>,
    assignment: bool,
    assign_to: String,
    assign_value: Value,
}

struct Operator {
    name: String,
    num_args: usize,
    func: fn(Vec<String>) -> OpResult,
}

fn strlen(str: &str) -> usize {
    str.graphemes(true).count()
}

fn get_operator(op: &str) -> Option<Operator> {
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

struct Scope {}

struct Interpreter {
    word_separator: char,
    scope: Scope,
}

impl Interpreter {
    fn new(word_separator: char) -> Interpreter {
        Interpreter {
            scope: Scope {},
            word_separator,
        }
    }

    fn eval(&mut self, input: &str) -> Result<String, String> {
        let inputlist = input.split(self.word_separator).collect::<Vec<&str>>();
        // OP var1 var2
        match inputlist[..] {
            [op, var1, var2, ..] => {
                match get_operator(op) {
                    Some(operator) => {
                        if operator.num_args != inputlist.len() - 1 {
                            return Err(format!("Invalid number of arguments for operator {}", op));
                        }
                        let result = (operator.func)([var1, var2].map(|x| x.to_string()).to_vec());
                        match result.result {
                            Ok(value) => Ok(value.value),
                            Err(err) => Err(err),
                        }
                    },
                    None => Err(format!("Invalid operator {}", op)),
                }
            },
            _ => Err(format!("{}", input)),
        }
    }
}

fn make_cache() -> Result<String, std::io::Error> {
    let system_cache_dir = dirs::cache_dir().unwrap();
    let cache_path = system_cache_dir.join("mangle");
    fs::create_dir_all(&cache_path)?;
    Ok(cache_path.to_str().unwrap().to_string())
}

struct Context {
    cache_dir: String,
}

fn repl(context: Context) {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    let cache_file_path = context.cache_dir.clone() + "/history";
    if rl.load_history(&cache_file_path).is_err() {
        println!("No previous history.");
    }
    let mut interpreter = Interpreter::new(' ');
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let result = match interpreter.eval(&line) {
                    Ok(result) => result,
                    Err(err) => err,
                };
                println!("Line: {}", result);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(&cache_file_path).unwrap();
}

