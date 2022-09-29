use anyhow::Result;
use chrono::NaiveDate;
use std::path::PathBuf;
use super::utils;

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

#[derive(Debug)]
pub struct Day {
    pub date: NaiveDate,
    events: Vec<String>,
    /* pub habits: Vec<Habit>,
    pub notes: Vec<String>,
    pub gratitudes: Vec<String>, */
}

impl Day {
    pub fn new(path: &PathBuf) -> Result<Self> {
        let ymd = utils::get_date_from_path(&path)?;
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
