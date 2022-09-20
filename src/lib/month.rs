use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use anyhow::Result;
use crate::day::{Day, Task};

pub struct Month {
    pub tasks: Vec<Task>,
    pub days: Vec<Day>,
}

impl Month {
    pub fn new(path: PathBuf) -> Result<Self> {
        let month = Month {
            tasks: vec![],
            days: vec![],
        };

        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        loop {
            let mut line = String::new();
            let len = reader.read_line(&mut line)?;

            let l = line.as_str();
            match l {
                l if l.starts_with("###") => println!("some hashes"),
                l if l.starts_with("- [ ]") => println!("a task"),
                _ => println!("another thing"),
            }
            if len == 0 {
                break;
            }
        }

        Ok(month)
    }
}
