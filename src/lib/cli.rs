use clap::{
    builder::{EnumValueParser, PathBufValueParser, ArgSettings},
    Arg, Command, ValueEnum, ValueHint, ArgMatches,
};
use core::fmt;
use chrono::Local;
use std::path::PathBuf;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(ValueEnum, Debug, Clone)]
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
            .value_parser(EnumValueParser::<Month>::new())
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
    pub fn from<I, T>(args: I) -> Self 
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

    pub fn get_month(&self) -> String {
        match self.matches.get_one::<Month>("month") {
            Some(month) => month.to_string(),
            None => {
                let current_month = Local::today().format("%B");
                current_month.to_string()
            }
        }
    }

    pub fn get_year(&self) -> String {
        match self.matches.get_one::<u16>("year") {
            Some(year) => year.to_string(),
            None => {
                let current_year = Local::today().format("%Y");
                current_year.to_string()
            }
        }
    }

    pub fn get_path(&self) -> PathBuf {
        self.matches.get_one::<PathBuf>("path").unwrap().to_path_buf()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;

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
        let cli = Cli::from(vec!["monsum", "--year", "2023", "-m", "february"]);

        let year = cli.matches.get_one::<u16>("year");
        assert!(matches!(year, Some(2023)));

        let month = cli.matches.get_one("month");
        assert!(matches!(month, Some(Month::February)));

        let path = cli.matches.get_one::<PathBuf>("path").unwrap();
        let path_str = path.clone().into_os_string();
        assert_eq!(path_str, ".");
    }

    #[test]
    fn get_month() {
        let cli = Cli::from(vec!["monsum", "-m", "february"]);
        assert_eq!(cli.get_month(), String::from("February"));

        let cli = Cli::from(vec!["monsum"]);
        let current_month = Local::today().format("%B");
        assert_eq!(cli.get_month(), current_month.to_string());
    }

    #[test]
    fn month_case_insensitive() {
        let cli = Cli::from(vec!["monsum", "-m", "FEBRUARY"]);
        assert_eq!(cli.get_month(), String::from("February"));

        let cli = Cli::from(vec!["monsum", "-m", "February"]);
        assert_eq!(cli.get_month(), String::from("February"));
    }

    #[test]
    fn get_year() {
        let cli = Cli::from(vec!["monsum", "-y", "2023"]);
        assert_eq!(cli.get_year(), String::from("2023"));

        let cli = Cli::from(vec!["monsum"]);
        let current_year = Local::today().format("%Y");
        assert_eq!(cli.get_year(), current_year.to_string());
    }

    #[test]
    fn get_path() {
        let cli = Cli::from(vec!["monsum", "-p", "~/code"]);
        assert_eq!(cli.get_path().into_os_string(), "~/code");

        let cli = Cli::from(vec!["monsum"]);
        assert_eq!(cli.get_path().into_os_string(), ".");
    }
}
