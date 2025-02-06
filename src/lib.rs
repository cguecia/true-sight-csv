pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

use csv::Reader;
use prettytable::{Cell, Row, Table};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn pretty_print(file_path: &std::path::PathBuf) -> Result<(), Box<dyn Error>> {
    // TODO: pretty print is currently using a reader to read the csv file
    // other things are going to read the file as well, this should really onnly be done 1 time
    let mut rdr = csv::Reader::from_path(file_path)?;
    let headers = rdr.headers()?;

    let mut table = Table::new();
    table.add_row(Row::from_iter(headers.iter().map(|h| Cell::new(h))));

    for result in rdr.records().take(10) {
        let record = result?;
        table.add_row(Row::from_iter(record.iter().map(|field| Cell::new(field))));
    }

    table.printstd();
    Ok(())
}

pub fn read_csv_chunks(path: &PathBuf, chunk_size: usize) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;
    // TODO: Check docks for ReaderBuilder to add more customiztion for example toggle has_headers bool true or false
    let mut rdr = Reader::from_reader(file);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
