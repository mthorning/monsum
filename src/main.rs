use anyhow::{Context,Result};
use lib::{Cli, Month};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let cli = Cli::new(args);

    let month = cli.get_month().context("Error getting month")?;

    let year = cli.get_year().context("Error getting year")?;

    let mut path = cli.get_path();


    path.push(format!("{} {}.md", month, year));

    let month = Month::new(&path).context("Error creating Month")?;
    month.print_tasks();
    month.print_days();

    Ok(())
}
