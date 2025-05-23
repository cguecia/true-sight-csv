mod args;

use args::TrueSightCsvArgs;
use clap::Parser;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;
use true_sight_csv::{
    prepare_csv_reader, CsvAggregator, CsvChunkIterator, EmptyCheck, NullLikeCheck, PatternCheck, WhiteSpaceOnlyCheck
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: TrueSightCsvArgs = TrueSightCsvArgs::parse();
    println!("Provided full path to file: {:?}", &args);

    let validated_path: &std::path::PathBuf = args.validate_csv_path()?;
    println!("Valid CSV path: {:?}", validated_path);

    let start_time = Instant::now();

    // Get both headers and reader
    let (found_headers, mut rdr) = prepare_csv_reader(validated_path)?;
    println!("Found headers: {:?}", found_headers);

    // Define chunk size
    let chunk_size = 1_000_000;

    let mut aggregator = CsvAggregator::new(found_headers.clone(), chunk_size);

    let chunk_iterator: CsvChunkIterator<'_, File> = CsvChunkIterator::new(rdr.records(), chunk_size);

    // Make the checkers thread-safe
    let null_check = Arc::new(NullLikeCheck::new());
    let empty_check = Arc::new(EmptyCheck::new());
    let white_space_only_check = Arc::new(WhiteSpaceOnlyCheck::new());

    // Initialize counters
    let total_row_count = Arc::new(AtomicUsize::new(0));
    let mut chunk_number = 0;

    // Main processing loop
    for chunk in chunk_iterator {
        match chunk {
            Ok(records) => {
                // Increment chunk counter
                chunk_number += 1;

                // Update total row count atomically
                let chunk_size = records.len();
                total_row_count.fetch_add(chunk_size, Ordering::Relaxed);

                // Create new counters for this chunk
                let null_counters = Arc::new(Mutex::new(HashMap::<usize, usize>::new()));
                let empty_counters = Arc::new(Mutex::new(HashMap::<usize, usize>::new()));
                let whhite_space_only_counters = Arc::new(Mutex::new(HashMap::<usize, usize>::new()));

                records.par_iter().for_each(|record| {
                    // Thread-local vectors to collect findings
                    let mut local_null_findings = Vec::new();
                    let mut local_empty_findings = Vec::new();
                    let mut local_white_space_only_findings = Vec::new();

                    // Process without holding locks
                    for (i, field) in record.iter().enumerate() {
                        if null_check.check(field) {
                            local_null_findings.push(i);
                        }

                        if empty_check.check(field) {
                            local_empty_findings.push(i);
                        }

                        if white_space_only_check.check(field) {
                            local_white_space_only_findings.push(i);
                        }
                    }

                    // Update counters only if needed
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

                    if !local_white_space_only_findings.is_empty() {
                        let mut white_space_only_map = whhite_space_only_counters.lock().unwrap();
                        for col in local_white_space_only_findings {
                            *white_space_only_map.entry(col).or_insert(0) += 1;
                        }
                    }
                });

                println!(
                    "\nProcessed chunk #{} with {} rows",
                    chunk_number, chunk_size
                );

                // Print statistics for this chunk
                println!("--- Statistics for chunk {}:", chunk_number);

                let null_map = null_counters.lock().unwrap();
                if !null_map.is_empty() {
                    println!("NULL-like values:");

                    for (col, count) in null_map.iter().filter(|(_, &count)| count > 0) {
                        let header_name = if *col < found_headers.len() {
                            &found_headers[*col]
                        } else {
                            "Unkown Column"
                        };

                        println!(
                            "   col_{} column_name={}: {} NULL-like values",
                            col, header_name, count
                        );
                    }
                } else {
                    println!("No NULL-like values found in this chunk");
                }

                let empty_map = empty_counters.lock().unwrap();
                if !empty_map.is_empty() {
                    println!("Empty values:");
                    for (col, count) in empty_map.iter().filter(|(_, &count)| count > 0) {
                        let header_name = if *col < found_headers.len() {
                            &found_headers[*col]
                        } else {
                            "Unkown Column"
                        };

                        println!(
                            "   col_{} column_name={}: {} empty values",
                            col, header_name, count
                        );
                    }
                } else {
                    println!("No empty values found in this chunk");
                }

                let white_space_only_map = whhite_space_only_counters.lock().unwrap();
                if !white_space_only_map.is_empty() {
                    println!("White Space Only values:");
                    for (col, count) in white_space_only_map.iter().filter(|(_, &count)| count > 0) {
                        let header_name = if *col < found_headers.len() {
                            &found_headers[*col]
                        } else {
                            "Unkown Column"
                        };

                        println!(
                            "   col_{} column_name={}: {} white space only values",
                            col, header_name, count
                        );
                    }
                } else {
                    println!("No white space only values found in this chunk");
                }


                aggregator.add_chunk_results(&null_map, &empty_map, &white_space_only_map, chunk_size);
            }
            Err(e) => {
                eprintln!("Error reading CSV: {}", e);
                break;
            }
        }
    }

        // Calculate elapsed time
        let elapsed_time = start_time.elapsed();

        // Set the processing time in the aggregator
        aggregator.set_processing_time(elapsed_time);
    

    // Final summary after all chunks are processed
    println!("\n=== PROCESSING COMPLETE ===");
    println!(
        "Total rows processed: {}",
        total_row_count.load(Ordering::Relaxed)
    );
    println!("Total chunks processed: {}", chunk_number);
    println!("Processing time: {:?}", elapsed_time);
    
    // Generate and print the aggregated report
    let aggregated_report = aggregator.generate_report();
    println!("{}", aggregated_report);
    println!("Total chunks processed: {}", chunk_number);

    Ok(())
}

//read_csv_chunks(validated_path, 2);

//let mut reader = csv::Reader::from_path(validated_path)?;

// Print all records
//for result in reader.records() {
//    let record = result?;
//    println!("{:?}", record);
//}

// Print in formatted table stdout
//pretty_print(validated_path)?;

// After processing all chunks, print the total row count
