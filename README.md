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


## Example CLI input with Outpup
```
Provided full path to file: TrueSightCsvArgs { file_full_path: "\\true-sight-csv\\tests\\sample-warehouse-data.csv" }
Valid CSV path: "\\true-sight-csv\\tests\\sample-warehouse-data.csv"
Found headers: ["customer_id", "order_date", "product_sku", "quantity", "unit_price", "shipping_zip", "email", "last_updated_timestamp", ""]
Chunk read with 12 records

Processed chunk #1 with 12 rows
--- Statistics for chunk 1:
NULL-like values:
   col_0 column_name=customer_id: 5 NULL-like values
   col_3 column_name=quantity: 2 NULL-like values
   col_2 column_name=product_sku: 2 NULL-like values
   col_1 column_name=order_date: 2 NULL-like values
   col_4 column_name=unit_price: 1 NULL-like values
   col_7 column_name=last_updated_timestamp: 1 NULL-like values
   col_5 column_name=shipping_zip: 1 NULL-like values
   col_8 column_name=: 1 NULL-like values
Empty values:
   col_2 column_name=product_sku: 3 empty values
   col_3 column_name=quantity: 3 empty values
   col_4 column_name=unit_price: 3 empty values
   col_8 column_name=: 9 empty values
   col_6 column_name=email: 7 empty values
   col_1 column_name=order_date: 1 empty values
   col_7 column_name=last_updated_timestamp: 3 empty values
   col_5 column_name=shipping_zip: 3 empty values

=== PROCESSING COMPLETE ===
Total rows processed: 12
Total chunks processed: 1
```