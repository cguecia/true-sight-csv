# true-sight-csv

A Rust-based CSV analysis tool that reveals hidden patterns and potential data quality issues in your CSV files, just as the True Sight spell reveals what's hidden from normal view.

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
Found headers: ["customer_id", "order_date", "product_sku", "quantity", "unit_price", "shipping_zip", "email", "last_updated_timestamp", ""]
Chunk read with 12 records

Chunk read with 12 records

Processed chunk #1 with 12 rows
--- Statistics for chunk 1:
NULL-like values:
   col_4 column_name=unit_price: 1 NULL-like values
   col_7 column_name=last_updated_timestamp: 1 NULL-like values
   col_2 column_name=product_sku: 2 NULL-like values
   col_1 column_name=order_date: 2 NULL-like values
   col_8 column_name=: 1 NULL-like values
   col_5 column_name=shipping_zip: 1 NULL-like values
   col_0 column_name=customer_id: 5 NULL-like values
   col_3 column_name=quantity: 2 NULL-like values
Empty values:
   col_1 column_name=order_date: 1 empty values
   col_2 column_name=product_sku: 3 empty values
   col_6 column_name=email: 7 empty values
   col_7 column_name=last_updated_timestamp: 3 empty values
   col_5 column_name=shipping_zip: 3 empty values
   col_4 column_name=unit_price: 3 empty values
   col_8 column_name=: 9 empty values
   col_3 column_name=quantity: 3 empty values

=== PROCESSING COMPLETE ===
Total rows processed: 12
Total chunks processed: 1
Processing time: 8.1017ms

=== CSV QUALITY REPORT ===
Total rows processed: 12
Total columns: 9

Chunk size used: 1000000 rows
Processing time: 0s 8ms
Processing rate: 1500.00 rows/second
COLUMN STATISTICS:
Column 0 ('customer_id'):
  NULL-like values: 5 (41.67%)
  Empty values: 0 (0.00%)

Column 1 ('order_date'):
  NULL-like values: 2 (16.67%)
  Empty values: 1 (8.33%)

Column 2 ('product_sku'):
  NULL-like values: 2 (16.67%)
  Empty values: 3 (25.00%)

Column 3 ('quantity'):
  NULL-like values: 2 (16.67%)
  Empty values: 3 (25.00%)

Column 4 ('unit_price'):
  NULL-like values: 1 (8.33%)
  Empty values: 3 (25.00%)

Column 5 ('shipping_zip'):
  NULL-like values: 1 (8.33%)
  Empty values: 3 (25.00%)

Column 6 ('email'):
  NULL-like values: 0 (0.00%)
  Empty values: 7 (58.33%)

Column 7 ('last_updated_timestamp'):
  NULL-like values: 1 (8.33%)
  Empty values: 3 (25.00%)

Column 8 (''):
  NULL-like values: 1 (8.33%)
  Empty values: 9 (75.00%)


Total chunks processed: 1
```


## Example CLI input with Output on larger dataset 941,597 rows
```
Provided full path to file: TrueSightCsvArgs { file_full_path: "\\true-sight-csv\\tests\\movies.csv" }
Valid CSV path: "\\true-sight-csv\\tests\\movies.csv"
Found headers: ["id", "name", "date", "tagline", "description", "minute", "rating"] 
Chunk read with 941597 records

Processed chunk #1 with 941597 rows
--- Statistics for chunk 1:
NULL-like values:
   col_1 column_name=name: 12 NULL-like values       
   col_4 column_name=description: 2 NULL-like values 
   col_3 column_name=tagline: 20 NULL-like values    
Empty values:
   col_3 column_name=tagline: 802193 empty values    
   col_5 column_name=minute: 181570 empty values     
   col_2 column_name=date: 91913 empty values        
   col_6 column_name=rating: 850598 empty values     
   col_4 column_name=description: 160810 empty values

=== PROCESSING COMPLETE ===
Total rows processed: 941597
Total chunks processed: 1   
Processing time: 7.1727904s 

=== CSV QUALITY REPORT ===
Total rows processed: 941597
Total columns: 7

Chunk size used: 1000000 rows
Processing time: 7s 172ms
Processing rate: 131287.93 rows/second
COLUMN STATISTICS:
Column 0 ('id'):
  NULL-like values: 0 (0.00%)
  Empty values: 0 (0.00%)

Column 1 ('name'):
  NULL-like values: 12 (0.00%)
  Empty values: 0 (0.00%)

Column 2 ('date'):
  NULL-like values: 0 (0.00%)
  Empty values: 91913 (9.76%)

Column 3 ('tagline'):
  NULL-like values: 20 (0.00%)
  Empty values: 802193 (85.19%)

Column 4 ('description'):
  NULL-like values: 2 (0.00%)
  Empty values: 160810 (17.08%)

Column 5 ('minute'):
  NULL-like values: 0 (0.00%)
  Empty values: 181570 (19.28%)

Column 6 ('rating'):
  NULL-like values: 0 (0.00%)
  Empty values: 850598 (90.34%)


Total chunks processed: 1

```

