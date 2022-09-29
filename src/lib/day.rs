use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use std::path::PathBuf;

/* pub enum Period {
    Morning,
    Afternoon,
    Evening,
} */

/* pub struct Habit {
    pub value: String,
    pub completed: bool,
    pub period: Period,
} */

pub struct Day {
    pub date: NaiveDate,
    events: Vec<String>,
    /* pub habits: Vec<Habit>,
    pub notes: Vec<String>,
    pub gratitudes: Vec<String>, */
}

impl Day {
    pub fn new(path: &PathBuf) -> Result<Self> {
        let ymd = get_date_from_path(&path)?;
        let (y, m, d) = ymd;
        Ok(Day {
            date: NaiveDate::from_ymd(y, m, d),
            events: Vec::new(),
        })
    }

    pub fn add_event(&mut self, event: String) {
        self.events.push(event);
    }
}

fn get_date_from_path(path: &PathBuf) -> Result<(i32, u32, u32)> {
    lazy_static! {
        static ref RE: Regex =
            RegexBuilder::new(r"^Daily Notes/(?P<year>\d{4})-(?P<month>\d{2})-(?P<date>\d{2})$")
                .build().unwrap();
    }

    if let Some(capture) = RE.captures(path.to_str().unwrap()) {

        let year = capture
            .name("year")
            .unwrap()
            .as_str()
            .parse::<i32>()?;

        let month = capture
            .name("month")
            .unwrap()
            .as_str()
            .parse::<u32>()?;

        let date = capture
            .name("date")
            .unwrap()
            .as_str()
            .parse::<u32>()?;

        return Ok((year, month, date));
    }

    Err(anyhow!("Couldn't get a date from path"))
}

#[cfg(test)]
mod get_date_tests {
    use super::get_date_from_path;
    use std::path::PathBuf;

    #[test]
    fn gets_a_task_from_a_line() {
        assert_eq!(
            get_date_from_path(&PathBuf::from("Daily Notes/2022-09-29")).unwrap(),
            (2022, 09, 29)
        );
    }

    #[test]
    fn gets_a_none_if_no_path() {
        assert!(get_date_from_path(&PathBuf::from("Daily Notes/20")).is_err())
    }
}
