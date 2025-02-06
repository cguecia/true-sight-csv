mod args;

use args::TrueSightCsvArgs;
use clap::Parser;
use csv::ReaderBuilder;
use std::fs::File;
use true_sight_csv::CsvChunkIterator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: TrueSightCsvArgs = TrueSightCsvArgs::parse();
    println!("Provided full path to file: {:?}", &args);

    let validated_path: &std::path::PathBuf = args.validate_csv_path()?;
    println!("Valid CSV path: {:?}", validated_path);

    // Open the file and create a Reader
    let file: File = File::open(validated_path)?;
    let mut rdr: csv::Reader<File> = ReaderBuilder::new().from_reader(file);

    let chunk_iterator: CsvChunkIterator<'_, File> = CsvChunkIterator::new(rdr.records(), 100_000);

    // Initialize a counter for the total row count
    let mut total_row_count = 0;

    // Iterate over the chunks and process them
    for chunk in chunk_iterator {
        match chunk {
            Ok(records) => {
                total_row_count += records.len();
                // Process the chunk here (e.g., perform QA checks)
                println!("Processed chunk of size {}", records.len());
            }
            Err(e) => {
                eprintln!("Error reading CSV: {}", e);
                break;
            }
        }
    }

    // TODO: build out way to read in the csv in chunks if file is large
    //

    //read_csv_chunks(validated_path, 2);

    //let mut reader = csv::Reader::from_path(validated_path)?;

    // Print all records
    //for result in reader.records() {
    //    let record = result?;
    //    println!("{:?}", record);
    //}

    // Print in formatted table stdout
    //pretty_print(validated_path)?;

    // TODO: make the csv checker with different checks for nulls or NaN ect.
    // strategey pattern for adding new checks easier

    // After processing all chunks, print the total row count
    println!("Total row count: {}", total_row_count);

    Ok(())
}
