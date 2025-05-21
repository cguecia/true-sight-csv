# true-sight-csv

A Rust-based CSV analysis tool that reveals hidden patterns and potential data quality issues in your CSV files, just as the True Sight spell reveals what's hidden from normal view.

```

```

## Current Features
- CLI interface for analyzing CSV files
- Detects basic patterns in CSV data:
  - Empty fields
  - NULL values
  - Whitespace-only values

## Installation
```bash
# Clone this repository
git clone 

# Build the project
cd true-sight-csv
cargo build --release

```

## Example CLI input with Output
```
Provided full path to file: TrueSightCsvArgs { file_full_path: "\\true-sight-csv\\tests\\sample-warehouse-data.csv" }
Valid CSV path: "\\true-sight-csv\\tests\\sample-warehouse-data.csv"
FFound headers: ["customer_id", "order_date", "product_sku", "quantity", "unit_price", "shipping_zip", "email", "last_updated_timestamp", ""]
Chunk read with 12 records

Processed chunk #1 with 12 rows
--- Statistics for chunk 1:
NULL-like values:
   col_3 column_name=quantity: 2 NULL-like values
   col_8 column_name=: 1 NULL-like values
   col_5 column_name=shipping_zip: 1 NULL-like values
   col_1 column_name=order_date: 2 NULL-like values
   col_4 column_name=unit_price: 1 NULL-like values
   col_0 column_name=customer_id: 5 NULL-like values
   col_2 column_name=product_sku: 2 NULL-like values
   col_7 column_name=last_updated_timestamp: 1 NULL-like values
Empty values:
   col_2 column_name=product_sku: 3 empty values
   col_8 column_name=: 9 empty values
   col_1 column_name=order_date: 1 empty values
   col_5 column_name=shipping_zip: 3 empty values
   col_6 column_name=email: 7 empty values
   col_3 column_name=quantity: 3 empty values
   col_4 column_name=unit_price: 3 empty values
   col_7 column_name=last_updated_timestamp: 3 empty values
White Space Only values:
   col_8 column_name=: 1 white space only values
   col_2 column_name=product_sku: 2 white space only values
   col_7 column_name=last_updated_timestamp: 1 white space only values

=== PROCESSING COMPLETE ===
Total rows processed: 12
Total chunks processed: 1
Processing time: 6.5691ms

=== CSV QUALITY REPORT ===
Total rows processed: 12
Total columns: 9

Chunk size used: 1000000 rows
Processing time: 0s 6ms
Processing rate: 2000.00 rows/second
COLUMN STATISTICS:
Column 0 ('customer_id'):
  NULL-like values: 5 (41.67%)
  Empty values: 0 (0.00%)
  White-Space-Only values: 0 (0.00%)

Column 1 ('order_date'):
  NULL-like values: 2 (16.67%)
  Empty values: 1 (8.33%)
  White-Space-Only values: 0 (0.00%)

Column 2 ('product_sku'):
  NULL-like values: 2 (16.67%)
  Empty values: 3 (25.00%)
  White-Space-Only values: 2 (16.67%)

Column 3 ('quantity'):
  NULL-like values: 2 (16.67%)
  White-Space-Only values: 0 (0.00%)

Column 4 ('unit_price'):
  NULL-like values: 1 (8.33%)
  Empty values: 3 (25.00%)
  White-Space-Only values: 0 (0.00%)

Column 5 ('shipping_zip'):
  NULL-like values: 1 (8.33%)
  Empty values: 3 (25.00%)
  White-Space-Only values: 0 (0.00%)

Column 6 ('email'):
  NULL-like values: 0 (0.00%)
  Empty values: 7 (58.33%)
  White-Space-Only values: 0 (0.00%)

Column 7 ('last_updated_timestamp'):
  NULL-like values: 1 (8.33%)
  Empty values: 3 (25.00%)
  White-Space-Only values: 1 (8.33%)

Column 8 (''):
  NULL-like values: 1 (8.33%)
  Empty values: 9 (75.00%)
  White-Space-Only values: 1 (8.33%)
```


## Example CLI input with Output on larger dataset ~1,000,000 row CSV at 529 MB (554,823,961 bytes) with a Processing time: ~16s
data found here -> https://www.kaggle.com/datasets/asaniczka/tmdb-movies-dataset-2023-930k-movies?resource=download
```
Provided full path to file: TrueSightCsvArgs { file_full_path: "\\movies_datasets\\TMDB_movie_dataset_v11.csv" }
Valid CSV path: "\\movies_datasets\\TMDB_movie_dataset_v11.csv"
Found headers: ["id", "title", "vote_average", "vote_count", "status", "release_date", "revenue", "runtime", "adult", "backdrop_path", "budget", "homepage", "imdb_id", "original_language", "original_title", "overview", "popularity", "poster_path", "tagline", "genres", "production_companies", "production_countries", "spoken_languages", "keywords"]
Chunk read with 1000000 records

Processed chunk #1 with 1000000 rows
--- Statistics for chunk 1:
NULL-like values:
   col_12 column_name=imdb_id: 450 NULL-like values
   col_14 column_name=original_title: 3 NULL-like values       
   col_9 column_name=backdrop_path: 485 NULL-like values       
   col_17 column_name=poster_path: 248 NULL-like values        
   col_1 column_name=title: 2 NULL-like values
   col_18 column_name=tagline: 8 NULL-like values
Empty values:
   col_22 column_name=spoken_languages: 424915 empty values    
   col_5 column_name=release_date: 194038 empty values
   col_12 column_name=imdb_id: 465256 empty values
   col_18 column_name=tagline: 847723 empty values
   col_19 column_name=genres: 403170 empty values
   col_14 column_name=original_title: 9 empty values
   col_17 column_name=poster_path: 337757 empty values
   col_9 column_name=backdrop_path: 719374 empty values        
   col_11 column_name=homepage: 892652 empty values
   col_21 column_name=production_countries: 450431 empty values
   col_20 column_name=production_companies: 545291 empty values
   col_15 column_name=overview: 208000 empty values
   col_1 column_name=title: 9 empty values
   col_23 column_name=keywords: 715813 empty values
White Space Only values:
   col_15 column_name=overview: 1163 white space only values
   col_11 column_name=homepage: 18 white space only values
   col_1 column_name=title: 3 white space only values
Chunk read with 225501 records

Processed chunk #2 with 225501 rows
--- Statistics for chunk 2:
NULL-like values:
   col_9 column_name=backdrop_path: 1 NULL-like values
   col_18 column_name=tagline: 1 NULL-like values
   col_1 column_name=title: 1 NULL-like values
   col_17 column_name=poster_path: 1 NULL-like values
Empty values:
   col_17 column_name=poster_path: 65659 empty values
   col_23 column_name=keywords: 188706 empty values
   col_15 column_name=overview: 52087 empty values
   col_1 column_name=title: 4 empty values
   col_5 column_name=release_date: 31675 empty values
   col_11 column_name=homepage: 204334 empty values
   col_22 column_name=spoken_languages: 115172 empty values
   col_12 column_name=imdb_id: 136555 empty values
   col_21 column_name=production_countries: 111332 empty values
   col_18 column_name=tagline: 205800 empty values
   col_19 column_name=genres: 105258 empty values
   col_14 column_name=original_title: 4 empty values
   col_9 column_name=backdrop_path: 187520 empty values
   col_20 column_name=production_companies: 137901 empty values
White Space Only values:
   col_11 column_name=homepage: 2 white space only values
   col_15 column_name=overview: 5 white space only values

=== PROCESSING COMPLETE ===
Total rows processed: 1225501
Total chunks processed: 2
Processing time: 16.4856018s

=== CSV QUALITY REPORT ===
Total rows processed: 1225501
Total columns: 24

Chunk size used: 1000000 rows
Processing time: 16s 485ms
Processing rate: 74340.37 rows/second
COLUMN STATISTICS:
col_0 ('id'):
  NULL-like values: 0 (0.00%)
  Empty values: 0 (0.00%)
  White-Space-Only values: 0 (0.00%)

col_1 ('title'):
  NULL-like values: 3 (0.00%)
  Empty values: 13 (0.00%)
  White-Space-Only values: 3 (0.00%)

col_2 ('vote_average'):
  NULL-like values: 0 (0.00%)
  Empty values: 0 (0.00%)
  White-Space-Only values: 0 (0.00%)

col_3 ('vote_count'):
  NULL-like values: 0 (0.00%)
  Empty values: 0 (0.00%)
  White-Space-Only values: 0 (0.00%)

col_4 ('status'):
  NULL-like values: 0 (0.00%)
  Empty values: 0 (0.00%)
  White-Space-Only values: 0 (0.00%)

col_5 ('release_date'):
  NULL-like values: 0 (0.00%)
  Empty values: 225713 (18.42%)
  White-Space-Only values: 0 (0.00%)

col_6 ('revenue'):
  NULL-like values: 0 (0.00%)
  Empty values: 0 (0.00%)
  White-Space-Only values: 0 (0.00%)

col_7 ('runtime'):
  NULL-like values: 0 (0.00%)
  Empty values: 0 (0.00%)
  White-Space-Only values: 0 (0.00%)

col_8 ('adult'):
  NULL-like values: 0 (0.00%)
  Empty values: 0 (0.00%)
  White-Space-Only values: 0 (0.00%)

col_9 ('backdrop_path'):
  NULL-like values: 486 (0.04%)
  Empty values: 906894 (74.00%)
  White-Space-Only values: 0 (0.00%)

col_10 ('budget'):
  NULL-like values: 0 (0.00%)
  Empty values: 0 (0.00%)
  White-Space-Only values: 0 (0.00%)

col_11 ('homepage'):
  NULL-like values: 0 (0.00%)
  Empty values: 1096986 (89.51%)
  White-Space-Only values: 20 (0.00%)

col_12 ('imdb_id'):
  NULL-like values: 450 (0.04%)
  Empty values: 601811 (49.11%)
  White-Space-Only values: 0 (0.00%)

col_13 ('original_language'):
  NULL-like values: 0 (0.00%)
  Empty values: 0 (0.00%)
  White-Space-Only values: 0 (0.00%)

col_14 ('original_title'):
  NULL-like values: 3 (0.00%)
  Empty values: 13 (0.00%)
  White-Space-Only values: 0 (0.00%)

col_15 ('overview'):
  NULL-like values: 0 (0.00%)
  Empty values: 260087 (21.22%)
  White-Space-Only values: 1168 (0.10%)

col_16 ('popularity'):
  NULL-like values: 0 (0.00%)
  Empty values: 0 (0.00%)
  White-Space-Only values: 0 (0.00%)

col_17 ('poster_path'):
  NULL-like values: 249 (0.02%)
  Empty values: 403416 (32.92%)
  White-Space-Only values: 0 (0.00%)

col_18 ('tagline'):
  NULL-like values: 9 (0.00%)
  Empty values: 1053523 (85.97%)
  White-Space-Only values: 0 (0.00%)

col_19 ('genres'):
  NULL-like values: 0 (0.00%)
  Empty values: 508428 (41.49%)
  White-Space-Only values: 0 (0.00%)

col_20 ('production_companies'):
  NULL-like values: 0 (0.00%)
  Empty values: 683192 (55.75%)
  White-Space-Only values: 0 (0.00%)

col_21 ('production_countries'):
  NULL-like values: 0 (0.00%)
  Empty values: 561763 (45.84%)
  White-Space-Only values: 0 (0.00%)

col_22 ('spoken_languages'):
  NULL-like values: 0 (0.00%)
  Empty values: 540087 (44.07%)
  White-Space-Only values: 0 (0.00%)

col_23 ('keywords'):
  NULL-like values: 0 (0.00%)
  Empty values: 904519 (73.81%)
  White-Space-Only values: 0 (0.00%)


Total chunks processed: 2
```

