use minigrep::{run, Config};
use std::{env, process};
/* Separation of concerns
Main is involved in configuring the program and calling a run
function in lib.rs which returns a Result.
*/
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        eprintln!("Usage: minigrep {{options}} {{file-path}}");
        process::exit(12);
    });
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
