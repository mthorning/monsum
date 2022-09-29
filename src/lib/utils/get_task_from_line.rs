use crate::month::Task;
use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};

pub fn get_task_from_line(line_str: &str) -> Result<Task> {
    lazy_static! {
        static ref RE: Regex = RegexBuilder::new(r"^- (.*)")
            .case_insensitive(true)
            .build()
            .unwrap();
    }

    if let Some(captures) = RE.captures(line_str) {
        let value = captures.name("value").unwrap().as_str().trim().to_owned();
        let checked = captures.name("checked").unwrap().as_str();
        return Ok(Task {
            value,
            completed: checked == "x" || checked == "X",
            date: None,
        });
    }

    Err(anyhow!("Couldn't get event from line"))
}

#[cfg(test)]
mod get_task_tests {
    use super::get_task_from_line;

    #[test]
    fn gets_a_task_from_a_line() {
        let expected = "This is my task";
        assert!(matches!(
            get_task_from_line(format!("- [ ] {expected}").as_str()),
            Ok(task) if task.value == expected && !task.completed
        ));
    }

    #[test]
    fn gets_a_completed_task_from_a_line() {
        let expected = "This is my task";
        assert!(matches!(
            get_task_from_line(format!("- [x] {expected}").as_str()),
            Ok(task) if task.completed
        ));
    }
    #[test]
    fn is_case_insensitive() {
        let expected = "This is my task";
        assert!(matches!(
            get_task_from_line(format!("- [X] {expected}").as_str()),
            Ok(task) if task.completed
        ));
    }

    #[test]
    fn gets_a_none_if_no_task() {
        assert!(get_task_from_line(format!("- [x").as_str()).is_err());
    }
}
