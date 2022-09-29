use crate::day::Day;

use super::utils;
use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::mem;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Task {
    pub value: String,
    pub completed: bool,
    pub date: Option<NaiveDate>,
}

pub struct Month {
    tasks: Vec<Task>,
    days: Vec<Day>,
}

enum LineMode {
    Normal,
    Day(Option<Day>),
}

impl Month {
    pub fn new(path: &PathBuf) -> Result<Self> {
        let mut month = Month {
            tasks: vec![],
            days: vec![],
        };

        let mut line_mode = LineMode::Normal;

        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        loop {
            let mut line = String::new();
            let len = reader.read_line(&mut line)?;
            let line_str = line.as_str().trim();

            match &mut line_mode {
                LineMode::Normal => {
                    // Found a task
                    if line_str.starts_with("- [") {
                        let task = utils::get_event_from_line(line_str)?;
                        month.tasks.push(task);

                    // Reached list of days
                    } else if line_str == "## Days" {
                        let _ = mem::replace(&mut line_mode, LineMode::Day(None));
                    }
                }
                LineMode::Day(possible_day) => match possible_day {
                    None => {
                        // Leaving day mode so soon!
                        if line_str.starts_with("## ") {
                            return Err(anyhow!("Didn't find any days!"));

                        // Found a date
                        } else if line_str.starts_with("### [[") {
                            let path = utils::get_path_from_line(line_str)?;
                            let day = Day::new(&path)?;
                            let _ = mem::replace(&mut line_mode, LineMode::Day(Some(day)));
                        }
                    }
                    Some(day) => {
                        // Leaving day mode (won't happen in current template)
                        if line_str.starts_with("## ") {
                            let _ = mem::replace(&mut line_mode, LineMode::Normal);

                        // Reached a new different date
                        } else if line_str.starts_with("### [[") {
                            let path = utils::get_path_from_line(line_str)?;
                            let new_day = Day::new(&path)?;
                            match mem::replace(&mut line_mode, LineMode::Day(Some(new_day))) {
                                LineMode::Day(current_day) => month.days.push(current_day.unwrap()),
                                _ => {}
                            }

                        // Found a task
                        } else if line_str.starts_with("- [") {
                            let mut task = utils::get_event_from_line(line_str)?;
                            task.date = Some(day.date);
                            month.tasks.push(task);

                        // Found an event
                        } else if line_str.starts_with("- ") {
                            day.add_event(String::from("new event"));
                        }
                    }
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

    pub fn print_days(&self) {
        for day in self.days.iter() {
            println!("{:?}", day);
        }
    }
}
