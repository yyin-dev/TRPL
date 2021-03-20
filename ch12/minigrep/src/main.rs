use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| { // closure
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


    // Because run returns Result<(), Box<dyn Error>>, we only care about the 
    // error result. Thus, we don't need unwrap_or_else.
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}


