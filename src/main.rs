use std::process;
use std::env;

use tasker_rs::Tasker;

fn main() {
    let tasker = Tasker::new().unwrap_or_else(|err| {
        eprintln!("Something went wrong: {}", err);
        process::exit(1);
    });
    
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = &args[1];
        match &command[..] {
            "list" | "l" => tasker.list(),
            "add" => tasker.add(&args[2..]),
            _ => tasker.list(),
        }
    } else {
        tasker.list();
    }
}

