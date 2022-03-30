use rustyline::error::ReadlineError;
use rustyline::Editor;

extern crate dirs;
use std::fs;

fn make_cache() -> Result<String, std::io::Error> {
    let system_cache_dir = dirs::cache_dir().unwrap();
    let cache_path = system_cache_dir.join("mangle");
    fs::create_dir_all(&cache_path)?;
    Ok(cache_path.to_str().unwrap().to_string())
}

// Struct that holds the cache dir
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
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("Line: {}", line);
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

fn main() {
    let context = Context {
        cache_dir: make_cache().unwrap(),
    };
    repl(context)
}
