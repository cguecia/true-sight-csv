use csv::{Reader, ReaderBuilder};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn prepare_csv_reader(path: &PathBuf) -> Result<(Vec<String>, Reader<File>), Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr: csv::Reader<File> = ReaderBuilder::new().from_reader(file);

    // Get the headers and convert them to owned Strings
    let headers: Vec<String> = rdr.headers()?.iter().map(|s| s.to_string()).collect();

    Ok((headers, rdr))
}

pub fn read_csv_chunks(rdr: &mut Reader<File>, chunk_size: usize) -> Result<(), Box<dyn Error>> {
    let mut records = rdr.records();

    // TODO: Think of a better way to send the process into the loop
    // Should this return the chunk or pointer to the chunk then continue like a python yield?

    while let Some(chunk) = records
        .by_ref()
        .take(chunk_size)
        .collect::<Result<Vec<_>, _>>()
        .ok()
    {
        if chunk.is_empty() {
            break;
        }
        println!("Processing chunk of {} records", chunk.len());
    }
    Ok(())
}

// Trying out making an iterator that can read in chunks with csv Reader
pub struct CsvChunkIterator<'a, R: Read> {
    records: csv::StringRecordsIter<'a, R>, // 'a is the lifetime specifier compiler is asking for this
    chunk_size: usize,
}

impl<'a, R: Read> CsvChunkIterator<'a, R> {
    pub fn new(records: csv::StringRecordsIter<'a, R>, chunk_size: usize) -> Self {
        CsvChunkIterator {
            records,
            chunk_size,
        }
    }
}

impl<R: Read> Iterator for CsvChunkIterator<'_, R> {
    type Item = Result<Vec<csv::StringRecord>, csv::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let chunk: Result<Vec<_>, _> = self.records.by_ref().take(self.chunk_size).collect(); // this essentially allows for the iterator not to reset and read from top of file again

        match chunk {
            Ok(records) if records.is_empty() => None, // End of iterator, no more chunks
            Ok(records) => {
                // Print a message when a chunk is read
                println!("Chunk read with {} records", records.len());
                Some(Ok(records))
            }
            Err(e) => Some(Err(e)), // Propagate the error if there was one
        }
    }
}

// Each check must be Send + Sync to work with Rayon
pub trait PatternCheck: Send + Sync {
    // Name of the check (for reporting)
    fn name(&self) -> &str;

    // The actual check logic
    fn check(&self, value: &str) -> bool;

    // Example of what this check looks for (for reporting)
    fn show_check_pattern(&self) -> &str;
}

// Empty Check Strategy
pub struct EmptyCheck;

impl EmptyCheck {
    pub fn new() -> Self {
        Self
    }
}

impl PatternCheck for EmptyCheck {
    fn name(&self) -> &str {
        "Empty"
    }
    fn check(&self, value: &str) -> bool {
        value.is_empty()
    }
    fn show_check_pattern(&self) -> &str {
        "Empty string \"\""
    }
}

// NULL Like Values Check Strategy
pub struct NullLikeCheck;

impl NullLikeCheck {
    const NULL_LIKE_VALUES: [&'static str; 5] = ["NULL", "N/A", "NA", "NONE", "NaN"]; // use const since only checks a few strings

    pub fn new() -> Self {
        Self
    }
}

impl PatternCheck for NullLikeCheck {
    fn name(&self) -> &str {
        "NULL_LIKE_VALUES"
    }

    fn check(&self, value: &str) -> bool {
        let trimmed = value.trim(); // Borrowed slice, no allocation
        Self::NULL_LIKE_VALUES
            .iter()
            .any(|&null| trimmed.eq_ignore_ascii_case(null))
    }

    fn show_check_pattern(&self) -> &str {
        "NULL, N/A, NA, None" //TODO: Can we ref the const here? avoid hardcode would need to change in trait as well
    }
}
