use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct TrueSightCsvArgs {
    /// The full path to the csv file to be inspected
    pub file_full_path: PathBuf,
}
