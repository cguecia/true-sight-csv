mod args;

use args::TrueSightCsvArgs;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: TrueSightCsvArgs = TrueSightCsvArgs::parse();
    println!("Provided full path to file: {:?}", &args);
    args.validate_csv_path()?;
    Ok(())
}
