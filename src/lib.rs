use std::env;
use std::fs::OpenOptions;
use std::io::{
    BufReader, Read
};

pub struct Tasker {
    pub tasks: Vec<String>
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

        Ok(Tasker { tasks })
    }
}
