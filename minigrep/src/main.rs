use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // `unwrap_or_else` behaves similarly to `unwrap`
    // if the `Result` is an `Ok` value, it returns the inner value `Ok` is wrapping
    // if the value is an `Err` value, this method calls the code in the supplied closure
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err); // `eprintln!` macro works just like normal `println!` but prints to `stderr` instead of `stdout`

        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
