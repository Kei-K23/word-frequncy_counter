# Word Frequency Counter CLI

## Overview

The Word Frequency Counter CLI is a Rust-based command-line tool designed to count the frequency of words in a text file. It offers features for case sensitivity and the option to ignore common words. This tool is useful for text analysis and data processing tasks.

## Features

- **File Input**: Reads content from a specified text file.
- **Case Insensitivity**: Option to count words without regard to their case.
- **Ignore Common Words**: Option to exclude common words such as "the", "and", etc., from the count.
- **Efficient Processing**: Handles large files efficiently using regular expressions and hash maps.

## Installation

To use this CLI tool, you need to have Rust installed on your system. If you don't have Rust installed, you can get it from [https://rustup.rs/](https://rustup.rs/).

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/word-frequency-counter.git
   cd word-frequency-counter
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. The compiled binary will be located in the `target/release` directory. You can move it to a directory in your `PATH` for easier access.

## Usage

Run the CLI tool with the following command:

```bash
word-frequency-counter --file <filename> [--case-insensitive] [--ignore-common]
```

## Parameters

- --file (or -f): Required. The path to the text file you want to analyze.
- --case-insensitive (or -c): Optional. If specified, the word count will be case-insensitive.
- --ignore-common (or -i): Optional. If specified, common words like "the", "and", etc., will be ignored in the count.

## Examples

1. Count words in a file with case insensitivity and ignoring common words:

```bash
word-frequency-counter --file input.txt --case-insensitive --ignore-common
```

2. Count words in a file with case sensitivity and no common words ignored:

```bash
word-frequency-counter --file input.txt
```
