use mangle::interpreter::Interpreter;

use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::Editor;

extern crate dirs;
use std::fs;
use std::path::Path;
use std::io::{self, BufRead};

fn main() {
    let args = Args::parse();
    if !args.filepath.is_empty() {
        if args.filepath == "-" {
            println!("Reading from stdin...");
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                let line = line.expect("Could not read line from standard in");
                println!("{}", line);
            }
            return;
        }
        if !Path::new(&args.filepath).is_file() {
            print!("Cannot read file at \"{}\"", args.filepath);
            return;
        }
        print!("Reading from file not implemented!");
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
/// 
/// You can pass a filename or - to read from stdin
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Separator
    #[clap(short = 'F', long, default_value_t = ' ')]
    field: char,

    /// File to read
    #[clap(default_value_t = String::from(""))]
    filepath: String,
}

pub fn make_cache() -> Result<String, std::io::Error> {
    let system_cache_dir = dirs::cache_dir().unwrap();
    let cache_path = system_cache_dir.join("mangle");
    fs::create_dir_all(&cache_path)?;
    Ok(cache_path.to_str().unwrap().to_string())
}

pub struct Context {
    pub cache_dir: String,
    pub word_separator: char,
}

pub fn repl(context: Context) {
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
