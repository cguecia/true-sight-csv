mod args;

use args::TrueSightCsvArgs;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: TrueSightCsvArgs = TrueSightCsvArgs::parse();
    println!("Provided full path to file: {:?}", &args);

    let validated_path: &std::path::PathBuf = args.validate_csv_path()?;
    println!("Valid CSV path: {:?}", validated_path);
    Ok(())
}
