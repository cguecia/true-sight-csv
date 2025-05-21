use csv::{Reader, ReaderBuilder};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::collections::HashMap;
use std::time::Duration;

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

pub struct WhiteSpaceOnlyCheck;

impl WhiteSpaceOnlyCheck {
    pub fn new() -> Self {
        Self
    }
}

impl PatternCheck for WhiteSpaceOnlyCheck {
    fn name(&self) -> &str {
        "WhiteSpaceOnlyCheck"
    }
    fn check(&self, value: &str) -> bool {
        value.trim().is_empty()
    }
    fn show_check_pattern(&self) -> &str {
        "WhiteSpaceOnlyCheck string ' ' "
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
        "NULL, N/A, NA, None, NaN" //TODO: Can we ref the const here? avoid hardcode would need to change in trait as well
    }
}


// Create a struct to hold statistics for each column
// Add #[derive(Clone)] to the struct
#[derive(Clone)]
pub struct ColumnStats {
    null_like_count: usize,
    empty_count: usize,
    white_space_only_count: usize
    // Add other statistics as needed (pattern matches, etc.)
}

// Also add Clone to CsvAggregator if needed
#[derive(Clone)]
pub struct CsvAggregator {
    headers: Vec<String>,
    column_stats: Vec<ColumnStats>,
    total_rows: usize,
    chunk_size: usize,        // Add this to store chunk size
    processing_time: Option<Duration>, // Add this to track processing time
}


impl CsvAggregator {
    // Initialize with headers
    pub fn new(headers: Vec<String>, chunk_size: usize) -> Self {
        let column_count = headers.len();
        let column_stats = vec![
            ColumnStats {
                null_like_count: 0,
                empty_count: 0,
                white_space_only_count: 0
            };
            column_count
        ];
        
        CsvAggregator {
            headers,
            column_stats,
            total_rows: 0,
            chunk_size,
            processing_time: None
        }
    }
    
    // Add chunk results to aggregator
    pub fn add_chunk_results(&mut self, 
                         null_map: &HashMap<usize, usize>, 
                         empty_map: &HashMap<usize, usize>,
                         white_space_only_map: &HashMap<usize, usize>,
                         chunk_size: usize) {
        // Update total row count
        self.total_rows += chunk_size;
        
        // Update null-like counts
        for (&col, &count) in null_map.iter() {
            if col < self.column_stats.len() {
                self.column_stats[col].null_like_count += count;
            }
        }
        
        // Update empty counts
        for (&col, &count) in empty_map.iter() {
            if col < self.column_stats.len() {
                self.column_stats[col].empty_count += count;
            }
        }
            
        // Update white_space_only_map counts
        for (&col, &count) in white_space_only_map.iter() {
            if col < self.column_stats.len() {
                self.column_stats[col].white_space_only_count += count;
            }
        }
    }

    pub    // Add method to set processing time
    fn set_processing_time(&mut self, duration: Duration) {
        self.processing_time = Some(duration);
    }
    
    // Generate final report
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str(&format!("\n=== CSV QUALITY REPORT ===\n"));
        report.push_str(&format!("Total rows processed: {}\n", self.total_rows));
        report.push_str(&format!("Total columns: {}\n\n", self.headers.len()));
        report.push_str(&format!("Chunk size used: {} rows\n", self.chunk_size));


        // Add processing time to report if available
        if let Some(duration) = self.processing_time {
            let seconds = duration.as_secs();
            let millis = duration.subsec_millis();
            
            // Format processing time nicely
            if seconds > 60 {
                let minutes = seconds / 60;
                let remaining_secs = seconds % 60;
                report.push_str(&format!("Processing time: {}m {}s {}ms\n", 
                                         minutes, remaining_secs, millis));
            } else {
                report.push_str(&format!("Processing time: {}s {}ms\n", seconds, millis));
            }
            
            // Add processing rate (rows per second)
            if seconds > 0 || millis > 0 {
                let total_seconds = seconds as f64 + (millis as f64 / 1000.0);
                let rows_per_second = self.total_rows as f64 / total_seconds;
                report.push_str(&format!("Processing rate: {:.2} rows/second\n", rows_per_second));
            }
        }
        
        report.push_str("COLUMN STATISTICS:\n");
        for (i, header) in self.headers.iter().enumerate() {
            let stats = &self.column_stats[i];
            
            // Skip columns with no issues if desired
            // if stats.null_like_count == 0 && stats.empty_count == 0 { continue; }
            
            report.push_str(&format!("Column {} ('{}'):\n", i, header));
            
            // Calculate percentages
            let null_percent = if self.total_rows > 0 {
                (stats.null_like_count as f64 / self.total_rows as f64) * 100.0
            } else {
                0.0
            };
            
            let empty_percent = if self.total_rows > 0 {
                (stats.empty_count as f64 / self.total_rows as f64) * 100.0
            } else {
                0.0
            };

            let white_space_only_percent = if self.total_rows > 0 {
                (stats.white_space_only_count as f64 / self.total_rows as f64) * 100.0
            } else {
                0.0
            };
            
            report.push_str(&format!("  NULL-like values: {} ({:.2}%)\n", 
                                    stats.null_like_count, null_percent));
                                    
            report.push_str(&format!("  Empty values: {} ({:.2}%)\n", 
                                    stats.empty_count, empty_percent));

            report.push_str(&format!("  White-Space-Only values: {} ({:.2}%)\n", 
                                    stats.white_space_only_count, white_space_only_percent));

            report.push_str("\n");
        }
        
        report
    }
}
