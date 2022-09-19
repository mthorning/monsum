use anyhow::Result;
use lib::Cli;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let cli = Cli::from(args);

    let month = cli.get_month();
    let year = cli.get_year();
    let mut path = cli.get_path();

    path.push(format!("{} {}", month, year));
    println!("{:?}", path.into_os_string());

    Ok(())
}
