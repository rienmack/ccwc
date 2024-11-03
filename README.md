# ccwc

`ccwc` is a command-line utility written in Rust that counts the number of bytes, lines, and words in a given file. It is similar to the Unix `wc` (word count) command but implemented in Rust for educational purposes.

## Features

- Count the number of bytes in a file.
- Count the number of lines in a file.
- Count the number of words in a file.
- Display all counts by default or selectively based on provided flags.

## Usage

```sh
ccwc [OPTIONS] <FILE>
```

### Options

- `-c`: Count the number of bytes in the file.
- `-l`: Count the number of lines in the file.
- `-w`: Count the number of words in the file.
- `-h`: Print help information.
- `-V`: Print version information.

### Examples

Count the number of lines, words, and bytes in `example.txt`:

```sh
ccwc example.txt
```

Count only the number of bytes in `example.txt`:

```sh
ccwc -c example.txt
```

Count only the number of lines in `example.txt`:

```sh
ccwc -l example.txt
```

Count only the number of words in `example.txt`:

```sh
ccwc -w example.txt
```

## Installation

To install `ccwc`, you need to have Rust and Cargo installed. You can then build the project using Cargo:

```sh
git clone https://github.com/yourusername/ccwc.git
cd ccwc
cargo build --release
```

The compiled binary will be located in the `target/release` directory.

## Acknowledgements

This project was inspired by the Unix `wc` command and serves as an educational example of how to implement similar functionality in Rust.

https://codingchallenges.fyi/challenges/challenge-wc

