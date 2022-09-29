use crate::day::Day;

use anyhow::Result;
use chrono::NaiveDate;
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use std::mem;

#[derive(Debug)]
struct Task {
    value: String,
    completed: bool,
    date: Option<NaiveDate>,
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
                        if let Some(task) = get_task_from_line(line_str) {
                            month.tasks.push(task);
                        }

                    // Reached list of days
                    } else if line_str == "## Days" {
                        let _ = mem::replace(&mut line_mode, LineMode::Day(None));
                    }
                }
                LineMode::Day(possible_day) => match possible_day {
                    None => {
                        // Leaving day mode (won't happen in current template)
                        if line_str.starts_with("## ") {
                            let _ = mem::replace(&mut line_mode, LineMode::Normal);

                        // Found a date
                        } else if line_str.starts_with("### [[") {
                            if let Some(path) = get_path_from_line(line_str) {
                                let day = Day::new(&path)?;
                                let _ = mem::replace(&mut line_mode, LineMode::Day(Some(day)));
                            }
                        }
                    }
                    Some(day) => {
                        // Leaving day mode (won't happen in current template)
                        if line_str.starts_with("## ") {
                            let _ = mem::replace(&mut line_mode, LineMode::Normal);

                        // Reached a new different date
                        } else if line_str.starts_with("### [[") {
                            if let Some(path) = get_path_from_line(line_str) {
                                let new_day = Day::new(&path)?;
                                let _ = mem::replace(&mut line_mode, LineMode::Day(Some(new_day)));
                            }

                        // Found a task
                        } else if line_str.starts_with("- [") {
                            if let Some(mut task) = get_task_from_line(line_str) {
                                task.date = Some(day.date);
                                month.tasks.push(task);
                            }

                        // Found an event
                        } else if line_str.starts_with("- ") {
                            // day.add_event(String::from("new event"));
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
}

fn get_task_from_line(line_str: &str) -> Option<Task> {
    lazy_static! {
        static ref RE: Regex = RegexBuilder::new(r"^- \[(?P<checked>(?:x|\s))\](?P<value>.*)")
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
            date: None,
        }
    })
}

#[cfg(test)]
mod get_task_tests {
    use super::get_task_from_line;

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

fn get_path_from_line(line_str: &str) -> Option<PathBuf> {
    lazy_static! {
        static ref RE: Regex = RegexBuilder::new(r"^### \[\[(Daily Notes/\d{4}-\d{2}-\d{2})")
            .build()
            .expect("Error creating path regex");
    }

    match RE.captures(line_str) {
        None => None,
        Some(captures) => captures.get(1).map(|path| PathBuf::from(path.as_str())),
    }
}

#[cfg(test)]
mod get_path_tests {
    use super::get_path_from_line;
    use std::path::PathBuf;

    #[test]
    fn gets_a_task_from_a_line() {
        let expected = "Daily Notes/2022-09-29";
        assert_eq!(
            get_path_from_line(&format!("### [[{expected}|29th (Thur)]]")).unwrap(),
            PathBuf::from(expected)
        );
    }

    #[test]
    fn gets_a_none_if_no_path() {
        assert!(matches!(
            get_path_from_line(format!("### [[").as_str()),
            None
        ));
    }
}
