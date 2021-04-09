use mygrep::Config;
use std::env;
use std::process;

fn main() {
    // let args: Vec<String> = env::args().collect(); //Old code, not requried now

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = mygrep::run(config) {
        eprintln!("Application error {}", e);
        process::exit(1);
    }
}
