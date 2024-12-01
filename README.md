# Chewbekka

Chewbekka is a Rust utility that extracts and summarizes Markdown files from a specified directory. It allows for both recursive and non-recursive extraction of Markdown files, implementing size limits and basic error handling. Features async bindings to local Ollama model.

## Requirements

- Ollama

## Features

- Recursively extract Markdown files from a specified directory.
- Summarize each file
- Summarize all summaries

## Installation

To build and run Chewbekka, make sure you have Rust and Cargo installed. You can install them from [rustup.rs](https://rustup.rs/).

Clone the repository and navigate into it:

```bash
git clone <repository-url>
cd chewbekka
```

Then, build the project:

```bash
cargo build --release
```

## Usage

Chewbekka is used via the command line. The basic command structure is as follows:

```bash
chewbekka summarize <path>
```

### Arguments

- `<path>`: The path to the directory containing Markdown files you wish to summarize.

### Example

```bash
chewbekka summarize /path/to/markdown/files
```

## Functionality Overview

### `extract_markdown_files_recursive`

This function extracts Markdown files recursively from a directory, returning a `HashMap` with file names as keys and their contents as values.

### `extract_markdown_files_non_recursive`

This function extracts Markdown files from a directory without recursion. It returns an error if a directory is found.

### `read_file_content`

Reads the content of a specified file and returns it as a string.

## Testing

The program includes unit tests to ensure functionality. You can run the tests with:

```bash
cargo test
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
