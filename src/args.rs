use clap::Parser;
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct TrueSightCsvArgs {
    /// The full path to the csv file to be inspected.
    pub file_full_path: PathBuf,

    /// The number of rows to use in a chunk. Default is 1_000_000.
    #[arg(long, default_value = "1000000")]
    pub row_chunk_size: usize,

    /// Disable parallel execution (default is parallel enabled).
    #[arg(long)]
    pub disable_parallel: bool,
}

impl TrueSightCsvArgs {
    pub fn validate_csv_path(&self) -> Result<&PathBuf, Box<dyn Error>> {
        // Check existence first
        if !self.file_full_path.exists() {
            return Err(format!("File does not exist: {}", self.file_full_path.display()).into());
        }

        // Check if it's actually a file (not a directory)
        if !self.file_full_path.is_file() {
            return Err(format!(
                "Path exists but is not a file: {}",
                self.file_full_path.display()
            )
            .into());
        }

        // Check file extension
        match self.file_full_path.extension() {
            Some(ext) if ext == "csv" => Ok(&self.file_full_path),
            _ => Err(format!("File must be a csv: {}", self.file_full_path.display()).into()),
        }
    }

    /// Returns true if parallel execution is enabled (default behavior)
    pub fn is_parallel_enabled(&self) -> bool {
        !self.disable_parallel
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_is_parallel_enabled() {
        // Default should be parallel enabled
        let args = TrueSightCsvArgs::try_parse_from(&["prog", "data.csv"]).unwrap();
        assert!(args.is_parallel_enabled());

        // With --disable-parallel flag, should be disabled
        let args =
            TrueSightCsvArgs::try_parse_from(&["prog", "data.csv", "--disable-parallel"]).unwrap();
        assert!(!args.is_parallel_enabled());
    }

    #[test]
    fn test_default_values() {
        let args = TrueSightCsvArgs::try_parse_from(&["prog", "data.csv"]).unwrap();
        assert_eq!(args.row_chunk_size, 1_000_000);
        assert_eq!(args.disable_parallel, false); // Default is parallel enabled
    }

    #[test]
    fn test_custom_chunk_size() {
        let args =
            TrueSightCsvArgs::try_parse_from(&["prog", "data.csv", "--row-chunk-size", "500000"])
                .unwrap();
        assert_eq!(args.row_chunk_size, 500_000);
        assert_eq!(args.disable_parallel, false); // Should still be default
    }

    #[test]
    fn test_disable_parallel() {
        let args =
            TrueSightCsvArgs::try_parse_from(&["prog", "data.csv", "--disable-parallel"]).unwrap();
        assert_eq!(args.row_chunk_size, 1_000_000); // Should still be default
        assert_eq!(args.disable_parallel, true);
    }

    #[test]
    fn test_both_custom_values() {
        let args = TrueSightCsvArgs::try_parse_from(&[
            "prog",
            "data.csv",
            "--row-chunk-size",
            "100",
            "--disable-parallel",
        ])
        .unwrap();
        assert_eq!(args.row_chunk_size, 100);
        assert_eq!(args.disable_parallel, true);
    }
}
