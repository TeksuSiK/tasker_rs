use std::env;
use std::fs::OpenOptions;
use std::io::{
    BufReader, BufWriter, Read, Write
};

use colored::*;

pub struct Tasker {
    pub tasks: Vec<String>,
    pub path: String,
}

impl Tasker {
    pub fn new() -> Result<Tasker, &'static str> {
          let home: String = match env::consts::OS {
            "linux" => {
                env::var("HOME")
                    .unwrap()
            },

            "windows" => {
                env::var("USERPROFILE")
                    .unwrap()
            },

            _ => { return Err("Your OS is nout supported") },
        };

        let path = format!("{}/.tasker", &home);

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)
            .expect("An error occurred while accessing tasker file");

        let mut buffer = BufReader::new(&file);

        let mut tasks = String::new();
 
        buffer.read_to_string(&mut tasks)
            .unwrap();
        
        let tasks = tasks.to_string()
            .lines()
            .map(str::to_string)
            .collect();

        Ok(Tasker { tasks, path })
    }

    pub fn list(&self) {
        for (number, task) in self.tasks.iter().enumerate() {
            if task.is_empty() {
                continue;
            }

            let number = (number + 1).to_string().bold();

            if task.starts_with("~") {
                let task = task.replace("~", "").strikethrough();
                println!("{} {}", number, task);
            } else {
                println!("{} {}", number, task);
            }
        }
    }

    pub fn add(&self, args: &[String]) {
        if args.is_empty() { 
            eprintln!("You can't add nothing to your list");
        }

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)
            .expect("An error occurred while accessing tasker file");

        let mut buffer = BufWriter::new(&file);
        
        let args = args.join(" ");
        
        buffer.write_all((args + "\n").as_bytes())
            .expect("An error occurred while writing data");
    }

    pub fn remove(&self, arg: &str) {
        if arg.is_empty() {
            eprintln!("You must enter which item you want do remove");
        }

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)
            .expect("An error occurred while accessing tasker file");

        let mut buffer = BufWriter::new(&file);

        for (number, task) in self.tasks.iter().enumerate() {
            if (number + 1).to_string() == arg.trim() {
                continue;
            }

            let task = format!{"{}\n", task};

            buffer.write_all(task.as_bytes())
                .expect("An error occurred while removing task");
        }
    }
}
