mod args;

use args::TrueSightCsvArgs;
use clap::Parser;

fn main() {
    let args: TrueSightCsvArgs = TrueSightCsvArgs::parse();
    println!("Provided full path to file: {:?}", &args);
}
