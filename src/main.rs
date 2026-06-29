use mangle::interpreter::Interpreter;

use clap::{self, Parser};
use rustyline::error::ReadlineError;
use rustyline::Editor;

extern crate dirs;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

fn run_source(interpreter: &mut Interpreter, source: &str) {
    match interpreter.eval(source) {
        Ok(result) if !result.is_empty() => println!("{}", result),
        Ok(_) => {}
        Err(error) => eprintln!("Error: {}", error),
    }
}

fn read_to_string(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

fn main() {
    let args = Args::parse();
    if !args.filepath.is_empty() {
        let source = if args.filepath == "-" {
            println!("Reading from stdin...");
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).unwrap_or_default();
            buffer
        } else {
            if !Path::new(&args.filepath).is_file() {
                panic!("Cannot read file at \"{}\"", args.filepath);
            }
            read_to_string(&args.filepath)
                .unwrap_or_else(|_| panic!("Cannot read file at \"{}\"", args.filepath))
        };
        let mut interpreter = Interpreter::new(args.field);
        run_source(&mut interpreter, &source);
        return;
    }
    let cache_dir = make_cache().unwrap();
    let context = Context {
        cache_dir,
        word_separator: args.field,
    };
    repl(context);
}

/// The letter counting based exolang that can interpret any type of text
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Word separator, only accepts single character
    #[clap(short = 'F', long, default_value_t = ' ')]
    field: char,

    /// File to read. You can also use '-' to read from stdin. Omiting this argument will launch the REPL
    #[clap(default_value_t = String::from(""))]
    filepath: String,
}

fn make_cache() -> Result<String, std::io::Error> {
    let system_cache_dir = dirs::cache_dir().unwrap();
    let cache_path = system_cache_dir.join("mangle");
    fs::create_dir_all(&cache_path)?;
    Ok(cache_path.to_str().unwrap().to_string())
}

struct Context {
    cache_dir: String,
    word_separator: char,
}

fn repl(context: Context) {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    let cache_file_path = context.cache_dir.clone() + "/history";
    if rl.load_history(&cache_file_path).is_err() {
        println!("No previous history.");
    }
    let mut interpreter = Interpreter::new(context.word_separator);
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                }
                rl.add_history_entry(line.as_str());
                let result = match interpreter.eval(&line) {
                    Ok(result) => result,
                    Err(err) => format!("Error: {}", err),
                };
                if !result.is_empty() {
                    println!("{}", result);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C exiting...");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D exiting...");
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
