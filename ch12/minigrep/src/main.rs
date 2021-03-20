use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // command line args
    let args: Vec<String> = env::args().collect();
    // args is Vec<String>, but parse_config accepts &[String]. Why this works?
    // Answer: "Deref coercion" is a convenience that Rust performs to functions
    // and methods. It works only on types that implement the Deref trait. It
    // converts a type implement the Deref trait into a reference to another 
    // type (&Vec<String> -> &[String] in this case). For example, &String can
    // be converted to &str, as String implements the Deref trait and it
    // returns str. 
    // Reference: https://stackoverflow.com/a/49322987/9057530
    let config = Config::new(&args).unwrap_or_else(|err| { // closure
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


