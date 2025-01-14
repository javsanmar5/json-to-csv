# JSON to CSV Converter

A simple Rust program that converts a JSON file to a CSV format.

## Overview

This project reads data from a JSON file and converts it to a CSV format. It uses the `serde` and `serde_json` crates for JSON parsing, and the `csv` crate for generating the CSV output.

## Requirements

- Rust (installed via [rustup](https://rustup.rs/))

## How to Use

1. Clone the repository or download the project.

   ```sh
   git clone https://github.com/javsanmar5/json-to-csv
   cd json-to-csv

2. Add your JSON file (e.g., data.json) in the project directory. Make sure your JSON structure matches the expected format.
Example of data.json:
    ```
    [
        {"name": "John", "age": 30, "city": "New York"},
        {"name": "Jane", "age": 25, "city": "London"},
        {"name": "Alice", "age": 28, "city": "Paris"}
    ]

3. Build and run the project:
    ```
    cargo run
This will print the CSV output to the terminal. If you'd like to save the output to a file, modify the main.rs file to write to a file instead of stdout.