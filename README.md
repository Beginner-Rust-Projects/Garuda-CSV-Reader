# Rust CSV Reader

A simple Rust application for reading and parsing CSV files.

## Features

- Reads and parses CSV files.
- Displays the contents of the CSV file to the console.

## Getting Started

### Prerequisites

Before running this application, ensure you have Rust and Cargo installed. You can install them by following the instructions at [rustup.rs](https://rustup.rs/).

### Installation

1. Clone the repository:

```bash
git clone https://github.com/yourusername/garuda-csv-reader.git
cd csv-reader
```

2. Build the application:

```bash
cargo build --release
```

### Usage

```bash
csv-reader <csv_file_path>
```

Replace `<csv_file_path>` with the path to the CSV file you want to read.

### Example

```bash
csv-reader ./sample.csv
```

This command will read and parse the contents of the `sample.csv` file and display the records to the console.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- This application utilizes the `csv` crate for reading and parsing CSV files in Rust.
- Special thanks to the Rust community for their contributions and support.

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## Important
Do not use Symbols in the CSV File, Only text can be interpreted
