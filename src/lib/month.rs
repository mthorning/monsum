use crate::day::Day;
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;

#[derive(Debug)]
struct Task {
    value: String,
    completed: bool,
}

pub struct Month {
    tasks: Vec<Task>,
    days: Vec<Day>,
}

enum LineMode {
    None,
    Task,
    Day,
}

impl Month {
    pub fn new(path: &PathBuf) -> Result<Self> {
        let mut month = Month {
            tasks: vec![],
            days: vec![],
        };

        let mut line_mode = LineMode::None;

        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        loop {
            let mut line = String::new();
            let len = reader.read_line(&mut line)?;
            let line_str = line.as_str().trim();

            match line_mode {
                LineMode::None => match line_str {
                    "## Tasks" => line_mode = LineMode::Task,
                    "## Days" => line_mode = LineMode::Day,
                    _ => (),
                },
                LineMode::Task => match line_str {
                    line_str if line_str.starts_with("- [") => month.tasks.push(
                        get_task_from_line(line_str).context("Error getting task from line")?,
                    ),
                    "## Days" => line_mode = LineMode::Day,
                    "---" => line_mode = LineMode::None,
                    _ => (),
                },
                LineMode::Day => match line_str {
                    "## Tasks" => line_mode = LineMode::Task,
                    "---" => line_mode = LineMode::None,
                    _ => (),
                },
            }

            if len == 0 {
                break;
            }
        }

        Ok(month)
    }

    pub fn print_tasks(&self) {
        for task in self.tasks.iter() {
            println!("{:?}", task);
        }
    }
}

fn get_task_from_line(line_str: &str) -> Result<Task> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^- \[(?P<checked>(x|\s))\]\s+(?P<value>.*)")
            .context("Error constructing task regex")?;
    }

    //TODO write some tests for this regex and work out wtf is going on!
    if let Some(captures) = RE.captures(line_str) {
        let value = captures.name("checked").unwrap().as_str().to_owned();
        let checked = captures.name("checked").unwrap().as_str();
        Ok(Task {
            value,
            completed: checked == "x",
        })
    }
}
