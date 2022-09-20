use anyhow::{Context, Result};
use chrono::Local;
use clap::{
    builder::{ArgSettings, EnumValueParser, PathBufValueParser},
    Arg, ArgMatches, Command, ValueEnum, ValueHint,
};
use core::fmt;
use enum_iterator::{all, Sequence};
use inquire::{Confirm, Select};
use std::path::PathBuf;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(ValueEnum, Debug, Clone, Sequence)]
enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

fn add_month_option(command: Command) -> Command {
    command.arg(
        Arg::new("month")
            .short('m')
            .long("month")
            .help("The month to summarise")
            .takes_value(true)
            .setting(ArgSettings::CaseInsensitive)
            .value_parser(EnumValueParser::<Month>::new()),
    )
}

fn add_year_option(command: Command) -> Command {
    command.arg(
        Arg::new("year")
            .short('y')
            .long("year")
            .help("The year with the month to summarise")
            .takes_value(true)
            .value_parser(clap::value_parser!(u16).range(2022..2030)),
    )
}

fn add_path_option(command: Command) -> Command {
    command.arg(
        Arg::new("path")
            .short('p')
            .long("path")
            .help("The path to the directory containing the notes")
            .value_hint(ValueHint::DirPath)
            .takes_value(true)
            .value_parser(PathBufValueParser::new())
            .default_value("."),
    )
}

pub struct Cli {
    matches: ArgMatches,
}

impl Cli {
    pub fn new<I, T>(args: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        let mut command = Command::new("monsum")
            .version(VERSION)
            .author("Matt Thorning")
            .about("Summarises my monthly notes");

        command = add_year_option(command);
        command = add_month_option(command);
        command = add_path_option(command);
        let matches = command.get_matches_from(args);

        Cli { matches }
    }

    pub fn get_month(&self) -> Result<String> {
        let month_options: Vec<String> = all::<Month>().map(|m| m.to_string()).collect();

        Ok(match self.matches.get_one::<Month>("month") {
            None => {
                let selected_month = Select::new("Select month", month_options)
                    .prompt()
                    .context("Error prompting for month")?
                    .to_owned();

                selected_month
            }
            Some(month) => month.to_string(),
        })
    }

    pub fn get_year(&self) -> Result<String> {
        Ok(match self.matches.get_one::<u16>("year") {
            None => {
                let current_year = Local::today().format("%Y").to_string();

                if Confirm::new("This year?").prompt().context("Error prompting for year")? {
                    current_year
                } else {
                        let current_year_num = current_year.parse::<u16>()?;
                        let year_options =
                            vec![current_year_num - 1, current_year_num, current_year_num + 1]
                                .into_iter()
                                .map(|y| format!("{y}"))
                                .collect();

                        Select::new("Select year", year_options)
                            .prompt()?
                            .to_owned()
                }
            }
            Some(year) => year.to_string(),
        })
    }

    pub fn get_path(&self) -> PathBuf {
        self.matches
            .get_one::<PathBuf>("path")
            .unwrap()
            .to_path_buf()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn cli_year_option_positive() {
        let mut command = Command::new("monsum");
        command = add_year_option(command);

        let matches = command
            .clone()
            .get_matches_from(vec!["monsum", "--year", "2022"]);
        let year_option: u16 = *matches.get_one("year").unwrap();
        assert_eq!(year_option, 2022);

        let matches = command.get_matches_from(vec!["monsum", "-y", "2029"]);
        let year_option: u16 = *matches.get_one("year").unwrap();
        assert_eq!(year_option, 2029);
    }

    #[test]
    fn cli_year_option_negative() {
        let mut command = Command::new("monsum");
        command = add_year_option(command);

        let result = command
            .clone()
            .try_get_matches_from(vec!["monsum", "--year", "2021"]);
        assert!(matches!(result, Err(_)));

        let result = command.try_get_matches_from(vec!["monsum", "--year", "2030"]);
        assert!(matches!(result, Err(_)));
    }

    #[test]
    fn cli_month_option_positive() {
        let mut command = Command::new("monsum");
        command = add_month_option(command);

        let matches = command
            .clone()
            .get_matches_from(vec!["monsum", "--month", "august"]);
        let month_option = matches.get_one::<Month>("month").unwrap().to_owned();
        assert!(matches!(month_option, Month::August));
        assert_eq!(month_option.to_string(), "August");

        let matches = command.get_matches_from(vec!["monsum", "--month", "october"]);
        let month_option = matches.get_one::<Month>("month").unwrap().to_owned();
        assert!(matches!(month_option, Month::October));
        assert_eq!(month_option.to_string(), "October");
    }

    #[test]
    fn cli_month_option_case_insensitive() {
        let mut command = Command::new("monsum");
        command = add_month_option(command);

        let matches = command.get_matches_from(vec!["monsum", "--month", "fEBRUARY"]);
        let month_option = matches.get_one::<Month>("month").unwrap().to_owned();
        assert_eq!(month_option.to_string(), String::from("February"));
    }

    #[test]
    fn cli_month_option_negative() {
        let mut command = Command::new("monsum");
        command = add_month_option(command);

        let result = command
            .clone()
            .try_get_matches_from(vec!["monsum", "--month", "Sep"]);
        assert!(matches!(result, Err(_)));

        let result = command.try_get_matches_from(vec!["monsum", "--month", "Orange"]);
        assert!(matches!(result, Err(_)));
    }

    #[test]
    fn cli_path_option_default() {
        let mut command = Command::new("monsum");
        command = add_path_option(command);

        let matches = command.clone().get_matches_from(vec!["monsum"]);
        let path_option = matches.get_one::<PathBuf>("path").unwrap().to_owned();
        assert_eq!(path_option.into_os_string(), ".");
    }

    #[test]
    fn cli_path_option_positive() {
        let mut command = Command::new("monsum");
        command = add_path_option(command);

        let matches = command
            .clone()
            .get_matches_from(vec!["monsum", "--path", "~/code/monsum"]);
        let path_option = matches.get_one::<PathBuf>("path").unwrap().to_owned();
        assert_eq!(path_option.into_os_string(), "~/code/monsum");

        let matches = command.get_matches_from(vec!["monsum", "--path", "/etc/"]);
        let path_option = matches.get_one::<PathBuf>("path").unwrap().to_owned();
        assert_eq!(path_option.into_os_string(), "/etc/");
    }

    #[test]
    fn cli_matches() {
        let cli = Cli::new(vec!["monsum", "--year", "2023", "-m", "february"]);

        let year = cli.matches.get_one::<u16>("year");
        assert!(matches!(year, Some(2023)));

        let month = cli.matches.get_one("month");
        assert!(matches!(month, Some(Month::February)));

        let path = cli.matches.get_one::<PathBuf>("path").unwrap();
        let path_str = path.clone().into_os_string();
        assert_eq!(path_str, ".");
    }
}
