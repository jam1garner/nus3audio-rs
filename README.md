# nus3audio-rs
![Rust](https://github.com/jam1garner/nus3audio-rs/workflows/Rust/badge.svg)

Command line tool for working with nus3audio archive files (re)written in rust. Supports all tier 1 rust platforms (Windows, MacOS, Linux).

Install from source:

```
cargo install --git https://github.com/jam1garner/nus3audio-rs
```

Help text:

```
nus3audio 1.1.8

USAGE:
    nus3audio [FLAGS] [OPTIONS] [--] [file]

ARGS:
    <file>    nus3audio file to open

FLAGS:
    -h, --help         Prints help information
    -j, --json         Prints the contents of the nus3audio file as json
    -n, --new          Creates a new nus3audio file instead of reading one in
    -p, --print        Prints the contents of the nus3audio file
    -t, --tonelabel    Generates a corresponding .tonelabel file
    -V, --version      Prints version information
    -v, --visual       Edit in visual mode

OPTIONS:
    -A, --append <NAME> <NEWFILE>
    -d, --delete <INDEX>...                 Delete file at INDEX in nus3audio file
    -i, --extract-id <FOLDER>...            Extract nus3audio contents with their ids to FOLDER
    -e, --extract-name <extract-name>...
            Extract nus3audio contents with their filenames to FOLDER

        --rebuild-id <FOLDER>               Rebuild nus3audio contents with ids from a FOLDER
    -R, --rebuild-name <FOLDER>             Rebuild nus3audio contents with filenames from a FOLDER
    -r, --replace <INDEX> <NEWFILE>
    -w, --write <write>...                  Write to FILE after performing all other operations
```

Requires:

* cargo, rustc, etc. installed (https://doc.rust-lang.org/cargo/getting-started/installation.html)

Build from source:
```
git clone https://github.com/jam1garner/nus3audio-rs.git && \
cd nus3audio-rs && \
cargo build --release
```
