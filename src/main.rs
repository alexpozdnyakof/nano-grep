use std::env;
use std::error::Error;
use std::process;

use nanogrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args);

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    // if let Err(e) = run(config) {
    //     println!("Application error: {e}");

    //     process::exit(1);
    // }
}



