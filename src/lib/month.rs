use crate::day::Day;

use anyhow::Result;
use chrono::NaiveDate;
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;

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
    Day(Option<PathBuf>),
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

            match &line_mode {
                LineMode::Normal => match line_str {
                    line_str if line_str.starts_with("- [") => {
                        if let Some(task) = get_task_from_line(line_str) {
                            month.tasks.push(task);
                        }
                    }
                    "## Days" => line_mode = LineMode::Day(None),
                    _ => (),
                },
                LineMode::Day(day) => match line_str {
                    line_str if line_str.starts_with("## ") => line_mode = LineMode::Normal,
                    line_str if line_str.starts_with("### [[") => {
                        if let Some(path) = get_path_from_line(line_str) {
                            line_mode = LineMode::Day(Some(path));
                        }
                    }
                    _ => match day {
                        Some(path) if line_str.starts_with("- [") => {
                            if let Some(mut task) = get_task_from_line(line_str) {
                                if let Some(ymd) = get_date_from_path(path) {
                                    let (y, m, d) = ymd;
                                    task.date = Some(NaiveDate::from_ymd(y, m, d));
                                    month.tasks.push(task);
                                }
                            }
                        }
                        Some(path) if line_str.starts_with("- ") => {
                            todo!();
                        }
                        _ => {}
                    },
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

fn get_date_from_path(path: &PathBuf) -> Option<(i32, u32, u32)> {
    lazy_static! {
        static ref RE: Regex =
            RegexBuilder::new(r"^Daily Notes/(?P<year>\d{4})-(?P<month>\d{2})-(?P<date>\d{2})$")
                .build()
                .expect("Error creating path regex");
    }

    RE.captures(path.to_str().unwrap()).map(|captures| {
        let year = captures
            .name("year")
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        let month = captures
            .name("month")
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();
        let date = captures
            .name("date")
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();

        (year, month, date)
    })
}

#[cfg(test)]
mod get_date_tests {
    use std::path::PathBuf;
    use super::get_date_from_path;

    #[test]
    fn gets_a_task_from_a_line() {
        assert_eq!(
            get_date_from_path(&PathBuf::from("Daily Notes/2022-09-29")).unwrap(),
            (2022, 09, 29)
        );
    }

    #[test]
    fn gets_a_none_if_no_path() {
        assert!(matches!(
            get_date_from_path(&PathBuf::from("Daily Notes/20")),
            None
        ));
    }
}
