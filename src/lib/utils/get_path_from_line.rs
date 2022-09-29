use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use std::path::PathBuf;

pub fn get_path_from_line(line_str: &str) -> Result<PathBuf> {
    lazy_static! {
        static ref RE: Regex = RegexBuilder::new(r"^### \[\[(Daily Notes/\d{4}-\d{2}-\d{2})")
            .build()
            .unwrap();
    }

    let captures = RE.captures(line_str);
    if captures.is_some() {
        if let Some(path_match) = captures.unwrap().get(1).take() {
            return Ok(PathBuf::from(path_match.as_str()));
        }
    }

    Err(anyhow!("Couldn't get path from line"))
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
        assert!(get_path_from_line(format!("### [[").as_str()).is_err());
    }
}
