# Resist Command

## Overview

resist is a command-line tool that reads from stdin and redirects the input to one of several file descriptors based on provided conductances. The likelihood of a file descriptor being chosen is proportional to its conductance.

## Compilation

To compile resist, you will need Rust's package manager, Cargo. Ensure you have Rust and Cargo installed on your system.

1. Clone the repository:on

```console
git clone https://github.com/nilp0inter/resist.git
cd resist
```

2. Compile the project:

```console
cargo build --release
```

The compiled binary will be located at `target/release/resist`.

## Installation

After compiling resist, you can install it on your system.

1. Copy the binary to a location in your PATH, such as `/usr/local/bin/`:

```console
cp target/release/resist /usr/local/bin/
```

2. (Optional) Copy the man page to your man pages directory (e.g., `/usr/local/man/man1/`) and update the man database:

```console
cp resist.1 /usr/local/man/man1/
mandb
```

## Usage

resist takes conductance values as arguments, each corresponding to a file descriptor starting from 1 (stdout). Higher conductance values increase the probability of a file descriptor being chosen for output.

### Example

To redirect input with conductances 30, 15, and 2 to three different files (file1.txt, file2.txt, and file3.txt), using file descriptors 1 (stdout), 2 (stderr), and 3, respectively:

```console
resist 30 15 2 1>file1.txt 2>file2.txt 3>file3.txt
```
