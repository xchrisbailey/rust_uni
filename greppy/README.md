# greppy
This is a simple command-line tool written in Rust that allows you to search for files matching a pattern and then search those files for a given string.

## Features
- Pattern matching: You can use glob patterns to specify which files to search, e.g., `*.txt`, `*.rs`, `src/**/*.rs`.
- String search: You can provide a string to search for within the matched files.
- Case sensitivity: You can choose whether to perform case-sensitive or case-insensitive searches.

## Installation
1.	Clone the repository:
```
git clone https://github.com/your-username/greppy.git
```

2.	Build the project:
```
cd greppy
cargo build --release
```

## Usage
```
greppy <pattern> <search_string> [options]
```

## Options:
- `-i`, `--ignore-case`: Ignore case sensitivity.
- `-h`, `-help`: Display help information.

## Example:
```
# Search for all files ending in ".txt" and find lines containing "hello"
greppy "*.txt" "hello"
```

```
# Search for all files in the "src" directory ending in ".rs" and find lines containing "function" (case-insensitive)
greppy "src/**/*.rs" "function" -i
```

## Contributing
Contributions are welcome! Feel free to submit issues or pull requests.
License

This project is licensed under the MIT License. See the LICENSE file for more information.
