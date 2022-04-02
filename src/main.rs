use mangle::interpreter::Interpreter;

use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::Editor;

extern crate dirs;
use std::fs;
use std::io::{self, BufRead, Error};
use std::path::Path;

/// Evaluate mangle code and print to stdout
fn eval_and_print(interpreter: &mut Interpreter, line: Result<String, Error>) {
    let line = line.expect("Could not read line from standard input");
    let result = match interpreter.eval(&line) {
        Ok(result) => result,
        Err(err) => err,
    };
    if !result.is_empty() {
        println!("{}", result);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where P: AsRef<Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args = Args::parse();
    if !args.filepath.is_empty() {
        if args.filepath == "-" {
            println!("Reading from stdin...");
            let mut interpreter = Interpreter::new(args.field);
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                eval_and_print(&mut interpreter, line);
            }
            return;
        }
        if !Path::new(&args.filepath).is_file() {
            // print!();
            panic!("Cannot read file at \"{}\"", args.filepath);
        }
        if let Ok(lines) = read_lines(args.filepath) {
            let mut interpreter = Interpreter::new(args.field);
            for line in lines {
                eval_and_print(&mut interpreter, line);
            }
        }
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
                let result = match interpreter.eval_line(&line) {
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
