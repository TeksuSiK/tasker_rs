use std::process;

use tasker_rs::Tasker;

fn main() {
    let tasker = Tasker::new().unwrap_or_else(|err| {
        eprintln!("Something went wrong: {}", err);
        process::exit(1);
    });
}

