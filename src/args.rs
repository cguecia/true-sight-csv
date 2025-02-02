use clap::Parser;
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct TrueSightCsvArgs {
    /// The full path to the csv file to be inspected
    pub file_full_path: PathBuf,
}

impl TrueSightCsvArgs {
    pub fn validate_csv_path(&self) -> Result<&PathBuf, Box<dyn Error>> {
        // Check existence first
        if !self.file_full_path.exists() {
            return Err("File does not exist".into());
        }

        // Check if it's actually a file (not a directory)
        if !self.file_full_path.is_file() {
            return Err("Path exists but is not a file".into());
        }

        // Check file extension
        match self.file_full_path.extension() {
            Some(ext) if ext == "csv" => Ok(&self.file_full_path),
            _ => Err("File must have .csv extension".into()),
        }
    }
}
