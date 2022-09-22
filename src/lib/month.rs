use crate::day::Day;
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
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
                    line_str if line_str.starts_with("- [") => {
                        if let Some(task) = get_task_from_line(line_str) {
                            month.tasks.push(task);
                        }
                    }
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

fn get_task_from_line(line_str: &str) -> Option<Task> {
    lazy_static! {
        static ref RE: Regex = RegexBuilder::new(r"^- \[(?P<checked>(x|\s))\](?P<value>.*)")
            .case_insensitive(true)
            .build()
            .expect("Error creating task regex");
    }

    RE.captures(line_str).map(|captures| {
        let value = captures.name("value").unwrap().as_str().trim().to_owned();
        let checked = captures.name("checked").unwrap().as_str();
        Task {
            value,
            completed: checked == "x" || checked == "X",
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_a_task_from_a_line() {
        let expected = "This is my task";
        assert!(matches!(
            get_task_from_line(format!("- [ ] {expected}").as_str()),
            Some(task) if task.value == expected && !task.completed
        ));
    }

    #[test]
    fn gets_a_completed_task_from_a_line() {
        let expected = "This is my task";
        assert!(matches!(
            get_task_from_line(format!("- [x] {expected}").as_str()),
            Some(task) if task.completed
        ));
    }
    #[test]
    fn is_case_insensitive() {
        let expected = "This is my task";
        assert!(matches!(
            get_task_from_line(format!("- [X] {expected}").as_str()),
            Some(task) if task.completed
        ));
    }

    #[test]
    fn gets_a_none_if_no_task() {
        assert!(matches!(get_task_from_line(format!("- [x").as_str()), None));
    }
}
