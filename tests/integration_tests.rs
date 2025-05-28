#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::path::PathBuf;
    use true_sight_csv::{
        prepare_csv_reader, process_csv_chunks, process_single_chunk, ChunkProcessingResult,
        CsvChunkIterator, EmptyCheck, NullLikeCheck, PatternCheck, ProcessingConfig,
        WhiteSpaceOnlyCheck,
    };

    // Helper function to get the path to a fixture file
    fn get_fixture_path(fixture_filename: &str) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests"); // Assuming fixtures are in a subdirectory of 'tests'
        path.push(fixture_filename);
        path
    }

    #[test]
    fn test_path_printing() {
        let test_path = get_fixture_path("sample-warehouse-data.csv");
        println!("{:?}", test_path); // Use double quotes, not single quotes

        // Or if you want it as a readable string:
        println!("{}", test_path.display());
    }

    #[test]
    fn test_null_like_checks() {
        let null_check = NullLikeCheck::new();

        for &null_value in &NullLikeCheck::NULL_LIKE_VALUES {
            assert!(null_check.check(&null_value))
        }
    }

    #[test]
    fn test_empty_checks() {
        let empty_check = EmptyCheck::new();

        assert!(empty_check.check(""));
        assert_ne!(empty_check.check(" "), true)
    }

    #[test]
    fn test_white_space_only_check() {
        let white_space_only = WhiteSpaceOnlyCheck::new();

        assert!(white_space_only.check("         "))
    }

    #[test]
    fn test_csv_chunk_iterator() {
        let test_path = get_fixture_path("sample-warehouse-data.csv");

        let (_headers, mut rdr) = prepare_csv_reader(&test_path).unwrap();
        let chunk_size = 3;
        let mut chunk_iterator: CsvChunkIterator<'_, File> =
            CsvChunkIterator::new(rdr.records(), chunk_size);

        let first_chunk = chunk_iterator.next().unwrap().unwrap();

        assert_eq!(first_chunk.len(), 3);

        // Verify the content of the first chunk
        // Record 1: 1001,1/15/2024,SKU123,5,29.99,94105,john.doe@email.com,2024-01-15T08:30:00Z,
        assert_eq!(first_chunk[0].get(0), Some("1001"));
        assert_eq!(first_chunk[0].get(1), Some("1/15/2024"));
        assert_eq!(first_chunk[0].get(2), Some("SKU123"));
        assert_eq!(first_chunk[0].get(3), Some("5"));
        assert_eq!(first_chunk[0].get(6), Some("john.doe@email.com"));

        // Record 2: 1002,1/15/2024,SKU456,2,49.99,60601,,2024-01-15T09:15:00Z,
        assert_eq!(first_chunk[1].get(0), Some("1002"));
        assert_eq!(first_chunk[1].get(2), Some("SKU456"));
        assert_eq!(first_chunk[1].get(6), Some("")); // Empty email field

        // Record 3: 1003,1/16/2024,SKU789,1,99.99,10001,alice.smith@email.com,2024-01-16T14:20:00Z,
        assert_eq!(first_chunk[2].get(0), Some("1003"));
        assert_eq!(first_chunk[2].get(2), Some("SKU789"));
        assert_eq!(first_chunk[2].get(6), Some("alice.smith@email.com"));

        // Test second chunk
        let second_chunk = chunk_iterator.next().unwrap().unwrap();
        assert_eq!(second_chunk.len(), 3);

        // Verify some fields from second chunk
        assert_eq!(second_chunk[0].get(0), Some("1004")); // First record of second chunk
        assert_eq!(second_chunk[1].get(0), Some("1005")); // Second record
        assert_eq!(second_chunk[2].get(0), Some("1005")); // Third record (with blanks)
        assert_eq!(second_chunk[2].get(2), Some("   ")); // Blank product_sku with spaces

        // Test third chunk
        let third_chunk = chunk_iterator.next().unwrap().unwrap();
        assert_eq!(third_chunk.len(), 3);

        // This chunk should contain the problematic records
        assert_eq!(third_chunk[0].get(0), Some("MISSING1005"));
        assert_eq!(third_chunk[1].get(0), Some("NULL")); // First NULL record
        assert_eq!(third_chunk[2].get(0), Some("NULL")); // Second NULL record

        // Test fourth chunk (should have remaining records)
        let fourth_chunk = chunk_iterator.next().unwrap().unwrap();
        assert_eq!(fourth_chunk.len(), 3);

        // Last records with various NULL patterns
        assert_eq!(fourth_chunk[0].get(0), Some("NULL"));
        assert_eq!(fourth_chunk[1].get(0), Some("NULL"));
        assert_eq!(
            fourth_chunk[2].get(0),
            Some("                  NULL                   ")
        );

        // No more chunks should be available
        assert!(chunk_iterator.next().is_none());
    }

    #[test]
    fn test_csv_chunk_iterator_comprehensive() {
        let test_path = get_fixture_path("sample-warehouse-data.csv");
        let (_headers, mut rdr) = prepare_csv_reader(&test_path).unwrap();
        let chunk_size = 4; // Different chunk size

        let chunk_iterator: CsvChunkIterator<'_, File> =
            CsvChunkIterator::new(rdr.records(), chunk_size);

        // Collect all chunks to verify total structure
        let all_chunks: Result<Vec<_>, _> = chunk_iterator.collect();
        let all_chunks = all_chunks.unwrap();

        // Your CSV has 12 data rows, so with chunk_size=4: 4+4+4 = 3 chunks
        assert_eq!(all_chunks.len(), 3);
        assert_eq!(all_chunks[0].len(), 4);
        assert_eq!(all_chunks[1].len(), 4);
        assert_eq!(all_chunks[2].len(), 4);

        // Verify first record of each chunk
        assert_eq!(all_chunks[0][0].get(0), Some("1001"));
        assert_eq!(all_chunks[1][0].get(0), Some("1005")); // First record of second chunk
        assert_eq!(all_chunks[2][0].get(0), Some("NULL")); // First record of third chunk
    }

    #[test]
    fn test_csv_chunk_iterator_edge_cases() {
        let test_path = get_fixture_path("sample-warehouse-data.csv");

        // Test with chunk size larger than total records
        let (_headers, mut rdr) = prepare_csv_reader(&test_path).unwrap();
        let mut chunk_iterator: CsvChunkIterator<'_, File> =
            CsvChunkIterator::new(rdr.records(), 20);

        let only_chunk = chunk_iterator.next().unwrap().unwrap();
        assert_eq!(only_chunk.len(), 12); // All records in one chunk
        assert!(chunk_iterator.next().is_none());

        // Test with chunk size of 1
        let (_headers, mut rdr) = prepare_csv_reader(&test_path).unwrap();
        let chunk_iterator: CsvChunkIterator<'_, File> = CsvChunkIterator::new(rdr.records(), 1);

        let all_single_chunks: Result<Vec<_>, _> = chunk_iterator.collect();
        let all_single_chunks = all_single_chunks.unwrap();

        assert_eq!(all_single_chunks.len(), 12); // 12 chunks of 1 record each
        for chunk in &all_single_chunks {
            assert_eq!(chunk.len(), 1);
        }
    }

    #[test]
    fn test_process_csv_chunks() {
        let test_path = get_fixture_path("sample-warehouse-data.csv");

        // Get both headers and reader
        let (found_headers, mut rdr) = prepare_csv_reader(&test_path).unwrap();
        println!("Found headers: {:?}", found_headers);

        // Define chunk size
        let chunk_size = 3;

        let config = ProcessingConfig {
            chunk_size,
            enable_parallel: false,
        };
        let chunk_iterator = CsvChunkIterator::new(rdr.records(), chunk_size);

        let results = process_csv_chunks(chunk_iterator, config).unwrap();

        assert_eq!(results.len(), 4); // 12 rows / 3 = 4 chunks

        // Check first chunk
        assert_eq!(results[0].chunk_number, 1);
        assert_eq!(results[0].rows_processed, 3);

        // Check that we found some empty values (from your CSV)
        let total_empty_found: usize = results
            .iter()
            .map(|r| r.empty_counts.values().sum::<usize>())
            .sum();
        assert!(
            total_empty_found == 32,
            "Should find 32 empty values in test CSV"
        );

        // Check that we found some NULL-like values
        let total_null_found: usize = results
            .iter()
            .map(|r| r.null_counts.values().sum::<usize>())
            .sum();
        assert!(
            total_null_found == 15,
            "Should find 15 NULL-like values in test CSV"
        );
    }
}
