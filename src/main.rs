mod args;

use args::TrueSightCsvArgs;
use clap::Parser;
use std::time::Instant;
use true_sight_csv::print_chunk_results_spark_style;
use true_sight_csv::{
    prepare_csv_reader, process_csv_chunks, CsvAggregator, CsvChunkIterator, ProcessingConfig,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: TrueSightCsvArgs = TrueSightCsvArgs::parse();
    println!("Provided full path to file: {:?}", &args);

    let validated_path = args.validate_csv_path()?;
    println!("Valid CSV path: {:?}", validated_path);

    let start_time = Instant::now();

    // Get both headers and reader
    let (found_headers, mut rdr) = prepare_csv_reader(validated_path)?;
    println!("Found headers: {:?}", found_headers);

    // Define chunk size
    let chunk_size = 1_000_000;
    let mut aggregator = CsvAggregator::new(found_headers.clone(), chunk_size);

    let config = ProcessingConfig {
        chunk_size,
        enable_parallel: true,
    };
    let chunk_iterator = CsvChunkIterator::new(rdr.records(), chunk_size);

    // Process all chunks
    let results = process_csv_chunks(chunk_iterator, config)?;

    // Extract totals from results
    let total_rows_processed: usize = results.iter().map(|r| r.rows_processed).sum();
    let total_chunks_processed = results.len();

    // Calculate data quality totals
    let total_null_values: usize = results
        .iter()
        .map(|r| r.null_counts.values().sum::<usize>())
        .sum();
    let total_empty_values: usize = results
        .iter()
        .map(|r| r.empty_counts.values().sum::<usize>())
        .sum();
    let total_whitespace_values: usize = results
        .iter()
        .map(|r| r.whitespace_counts.values().sum::<usize>())
        .sum();

    // Print Spark-style formatted results
    print_chunk_results_spark_style(&results, &found_headers);

    // Calculate elapsed time
    let elapsed_time = start_time.elapsed();

    // Set the processing time in the aggregator
    aggregator.set_processing_time(elapsed_time);

    // Add results to aggregator for potential report generation
    for result in &results {
        aggregator.add_chunk_results(
            &result.null_counts,
            &result.empty_counts,
            &result.whitespace_counts,
            result.rows_processed,
        );
    }

    // Final summary with correct totals
    println!("\n=== PROCESSING COMPLETE ===");
    println!("Total rows processed: {}", total_rows_processed);
    println!("Total chunks processed: {}", total_chunks_processed);
    println!("Processing time: {:?}", elapsed_time);

    // Additional data quality summary
    println!("\n=== CSV QUALITY REPORT ===");
    println!("Total rows processed: {}", total_rows_processed);
    println!("Total columns: {}", found_headers.len());
    println!("Total data quality issues found:");
    println!("  - NULL-like values: {}", total_null_values);
    println!("  - Empty values: {}", total_empty_values);
    println!("  - Whitespace-only values: {}", total_whitespace_values);
    println!(
        "  - Total issues: {}",
        total_null_values + total_empty_values + total_whitespace_values
    );

    // Performance metrics
    let rows_per_second = if elapsed_time.as_secs() > 0 {
        total_rows_processed as f64 / elapsed_time.as_secs_f64()
    } else {
        total_rows_processed as f64
    };
    println!("Processing rate: {:.0} rows/second", rows_per_second);

    // Data quality percentages
    let total_cells = total_rows_processed * found_headers.len();
    let total_issues = total_null_values + total_empty_values + total_whitespace_values;
    let quality_percentage = if total_cells > 0 {
        ((total_cells - total_issues) as f64 / total_cells as f64) * 100.0
    } else {
        100.0
    };
    println!(
        "Overall data quality: {:.2}% clean cells",
        quality_percentage
    );

    // let aggregated_report = aggregator.generate_report();
    // println!("\n{}", aggregated_report);

    Ok(())
}
