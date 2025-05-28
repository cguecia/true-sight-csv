use csv::{Reader, ReaderBuilder};
use rayon::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

pub mod formatter; // Add this line to declare the module

// Re-export the public functions for convenience
pub use formatter::{print_chunk_results_spark_style, SparkStyleFormatter};

pub fn prepare_csv_reader(path: &Path) -> Result<(Vec<String>, Reader<File>), Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr: csv::Reader<File> = ReaderBuilder::new().from_reader(file);

    // Get the headers and convert them to owned Strings
    let headers: Vec<String> = rdr.headers()?.iter().map(|s| s.to_string()).collect();

    Ok((headers, rdr))
}

#[test]
fn test_csv_headers() {
    use std::path::PathBuf;
    // TODO: Set up a temp csv file that gets used for tests and gets cleaned up
    let mut path_buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path_buf.push("tests");
    path_buf.push("sample-warehouse-data.csv");

    let (headers, _reader) = prepare_csv_reader(&path_buf).unwrap();

    assert_eq!(headers.len(), 9);
    let expected_headers = vec![
        "customer_id".to_string(),
        "order_date".to_string(),
        "product_sku".to_string(),
        "quantity".to_string(),
        "unit_price".to_string(),
        "shipping_zip".to_string(),
        "email".to_string(),
        "last_updated_timestamp".to_string(),
        "".to_string(),
    ];

    assert_eq!(headers, expected_headers);
    assert!(!headers.is_empty(), "Headers should not be empty");
    println!("Headers: {:?}", headers);
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
        !value.is_empty() && value.trim().is_empty()
    }
    fn show_check_pattern(&self) -> &str {
        "WhiteSpaceOnlyCheck string ' ' "
    }
}

// NULL Like Values Check Strategy
pub struct NullLikeCheck;

impl NullLikeCheck {
    pub const NULL_LIKE_VALUES: [&'static str; 5] = ["NULL", "N/A", "NA", "NONE", "NaN"]; // use const since only checks a few strings

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
#[derive(Clone)]
pub struct ColumnStats {
    null_like_count: usize,
    empty_count: usize,
    white_space_only_count: usize, // Add other statistics as needed (pattern matches, etc.)
}

#[derive(Clone)]
pub struct CsvAggregator {
    headers: Vec<String>,
    column_stats: Vec<ColumnStats>,
    total_rows: usize,
    chunk_size: usize,
    processing_time: Option<Duration>,
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
            processing_time: None,
        }
    }

    // Add chunk results to aggregator
    pub fn add_chunk_results(
        &mut self,
        null_map: &HashMap<usize, usize>,
        empty_map: &HashMap<usize, usize>,
        white_space_only_map: &HashMap<usize, usize>,
        chunk_size: usize,
    ) {
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

    pub fn set_processing_time(&mut self, duration: Duration) {
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
                report.push_str(&format!(
                    "Processing time: {}m {}s {}ms\n",
                    minutes, remaining_secs, millis
                ));
            } else {
                report.push_str(&format!("Processing time: {}s {}ms\n", seconds, millis));
            }

            // Add processing rate (rows per second)
            if seconds > 0 || millis > 0 {
                let total_seconds = seconds as f64 + (millis as f64 / 1000.0);
                let rows_per_second = self.total_rows as f64 / total_seconds;
                report.push_str(&format!(
                    "Processing rate: {:.2} rows/second\n",
                    rows_per_second
                ));
            }
        }

        report.push_str("COLUMN STATISTICS:\n");
        for (i, header) in self.headers.iter().enumerate() {
            let stats = &self.column_stats[i];

            // Skip columns with no issues if desired
            // if stats.null_like_count == 0 && stats.empty_count == 0 { continue; }

            report.push_str(&format!("col_{} ('{}'):\n", i, header));

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

            report.push_str(&format!(
                "  NULL-like values: {} ({:.2}%)\n",
                stats.null_like_count, null_percent
            ));

            report.push_str(&format!(
                "  Empty values: {} ({:.2}%)\n",
                stats.empty_count, empty_percent
            ));

            report.push_str(&format!(
                "  White-Space-Only values: {} ({:.2}%)\n",
                stats.white_space_only_count, white_space_only_percent
            ));

            report.push_str("\n");
        }

        report
    }
}

// Struct to hold processing results for a single chunk
#[derive(Debug, Clone)]
pub struct ChunkProcessingResult {
    pub chunk_number: usize,
    pub rows_processed: usize,
    pub null_counts: HashMap<usize, usize>,
    pub empty_counts: HashMap<usize, usize>,
    pub whitespace_counts: HashMap<usize, usize>,
}

// Struct to hold overall processing configuration
pub struct ProcessingConfig {
    pub chunk_size: usize,
    pub enable_parallel: bool,
}

impl Default for ProcessingConfig {
    fn default() -> Self {
        Self {
            chunk_size: 1_000_000,
            enable_parallel: true,
        }
    }
}

// Main processing function
pub fn process_csv_chunks<R: Read>(
    chunk_iterator: CsvChunkIterator<'_, R>,
    config: ProcessingConfig,
) -> Result<Vec<ChunkProcessingResult>, Box<dyn std::error::Error>> {
    let null_check = Arc::new(NullLikeCheck::new());
    let empty_check = Arc::new(EmptyCheck::new());
    let white_space_only_check = Arc::new(WhiteSpaceOnlyCheck::new());

    let mut results = Vec::new();
    let mut chunk_number = 0;

    for chunk in chunk_iterator {
        match chunk {
            Ok(records) => {
                chunk_number += 1;

                let result = process_single_chunk(
                    &records,
                    chunk_number,
                    &null_check,
                    &empty_check,
                    &white_space_only_check,
                    config.enable_parallel,
                )?;

                results.push(result);
            }
            Err(e) => {
                return Err(Box::new(e));
            }
        }
    }

    Ok(results)
}

// Process a single chunk
pub fn process_single_chunk(
    records: &[csv::StringRecord],
    chunk_number: usize,
    null_check: &Arc<NullLikeCheck>,
    empty_check: &Arc<EmptyCheck>,
    whitespace_check: &Arc<WhiteSpaceOnlyCheck>,
    enable_parallel: bool,
) -> Result<ChunkProcessingResult, Box<dyn std::error::Error>> {
    let null_counters = Arc::new(Mutex::new(HashMap::<usize, usize>::new()));
    let empty_counters = Arc::new(Mutex::new(HashMap::<usize, usize>::new()));
    let whitespace_counters = Arc::new(Mutex::new(HashMap::<usize, usize>::new()));

    if enable_parallel {
        records.par_iter().for_each(|record| {
            process_record(
                record,
                &null_counters,
                &empty_counters,
                &whitespace_counters,
                null_check,
                empty_check,
                whitespace_check,
            );
        });
    } else {
        records.iter().for_each(|record| {
            process_record(
                record,
                &null_counters,
                &empty_counters,
                &whitespace_counters,
                null_check,
                empty_check,
                whitespace_check,
            );
        });
    }

    // Extract results from Arc<Mutex<>>
    let null_counts = null_counters.lock().unwrap().clone();
    let empty_counts = empty_counters.lock().unwrap().clone();
    let whitespace_counts = whitespace_counters.lock().unwrap().clone();

    Ok(ChunkProcessingResult {
        chunk_number,
        rows_processed: records.len(),
        null_counts,
        empty_counts,
        whitespace_counts,
    })
}

// Process a single record - the core logic
fn process_record(
    record: &csv::StringRecord,
    null_counters: &Arc<Mutex<HashMap<usize, usize>>>,
    empty_counters: &Arc<Mutex<HashMap<usize, usize>>>,
    whitespace_counters: &Arc<Mutex<HashMap<usize, usize>>>,
    null_check: &Arc<NullLikeCheck>,
    empty_check: &Arc<EmptyCheck>,
    whitespace_check: &Arc<WhiteSpaceOnlyCheck>,
) {
    let mut local_null_findings = Vec::new();
    let mut local_empty_findings = Vec::new();
    let mut local_whitespace_findings = Vec::new();

    for (i, field) in record.iter().enumerate() {
        if null_check.check(field) {
            local_null_findings.push(i);
        }
        if empty_check.check(field) {
            local_empty_findings.push(i);
        }
        if whitespace_check.check(field) {
            local_whitespace_findings.push(i);
        }
    }

    // Update counters
    if !local_null_findings.is_empty() {
        let mut null_map = null_counters.lock().unwrap();
        for col in local_null_findings {
            *null_map.entry(col).or_insert(0) += 1;
        }
    }

    if !local_empty_findings.is_empty() {
        let mut empty_map = empty_counters.lock().unwrap();
        for col in local_empty_findings {
            *empty_map.entry(col).or_insert(0) += 1;
        }
    }

    if !local_whitespace_findings.is_empty() {
        let mut whitespace_map = whitespace_counters.lock().unwrap();
        for col in local_whitespace_findings {
            *whitespace_map.entry(col).or_insert(0) += 1;
        }
    }
}

// Print results function
pub fn print_chunk_results(result: &ChunkProcessingResult, headers: &[String]) {
    println!(
        "\nProcessed chunk #{} with {} rows",
        result.chunk_number, result.rows_processed
    );

    // Print statistics for this chunk
    println!("--- Statistics for chunk {}:", result.chunk_number);

    // NULL-like values
    if !result.null_counts.is_empty() {
        println!("NULL-like values:");
        for (col, count) in result.null_counts.iter().filter(|(_, &count)| count > 0) {
            let header_name = if *col < headers.len() {
                &headers[*col]
            } else {
                "Unknown Column"
            };

            println!(
                "   col_{} column_name={}: {} NULL-like values",
                col, header_name, count
            );
        }
    } else {
        println!("No NULL-like values found in this chunk");
    }

    // Empty values
    if !result.empty_counts.is_empty() {
        println!("Empty values:");
        for (col, count) in result.empty_counts.iter().filter(|(_, &count)| count > 0) {
            let header_name = if *col < headers.len() {
                &headers[*col]
            } else {
                "Unknown Column"
            };

            println!(
                "   col_{} column_name={}: {} empty values",
                col, header_name, count
            );
        }
    } else {
        println!("No empty values found in this chunk");
    }

    // White space only values
    if !result.whitespace_counts.is_empty() {
        println!("White Space Only values:");
        for (col, count) in result
            .whitespace_counts
            .iter()
            .filter(|(_, &count)| count > 0)
        {
            let header_name = if *col < headers.len() {
                &headers[*col]
            } else {
                "Unknown Column"
            };

            println!(
                "   col_{} column_name={}: {} white space only values",
                col, header_name, count
            );
        }
    } else {
        println!("No white space only values found in this chunk");
    }
}

// Aggregate results function
pub fn aggregate_results(
    results: &[ChunkProcessingResult],
) -> (
    HashMap<usize, usize>,
    HashMap<usize, usize>,
    HashMap<usize, usize>,
    usize,
) {
    let mut total_null_counts = HashMap::new();
    let mut total_empty_counts = HashMap::new();
    let mut total_whitespace_counts = HashMap::new();
    let mut total_rows = 0;

    for result in results {
        total_rows += result.rows_processed;

        for (col, count) in &result.null_counts {
            *total_null_counts.entry(*col).or_insert(0) += count;
        }
        for (col, count) in &result.empty_counts {
            *total_empty_counts.entry(*col).or_insert(0) += count;
        }
        for (col, count) in &result.whitespace_counts {
            *total_whitespace_counts.entry(*col).or_insert(0) += count;
        }
    }

    (
        total_null_counts,
        total_empty_counts,
        total_whitespace_counts,
        total_rows,
    )
}
