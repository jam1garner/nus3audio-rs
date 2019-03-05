# nus3audio-rs
Command line tool for working with nus3audio archive files (re)written in rust.

```
nus3audio 1.0
jam1garner
Tool for working with nus3audio files

USAGE:
    nus3audio [FLAGS] [OPTIONS] <file>

FLAGS:
    -h, --help       Prints help information
    -p, --print      Prints the contents of the nus3audio file
    -V, --version    Prints version information
    -v, --visual     Edit in visual mode

OPTIONS:
    -d, --delete <INDEX>...            Delete file at INDEX in nus3audio file
    -e, --extract <FOLDER>...          Extract nus3audio contents to FOLDER
    -r, --replace <INDEX> <NEWFILE>    Replaces a file at INDEX with NEWFILE
    -w, --write <FILE>...              Write to FILE after performing all other operations

ARGS:
    <file>    nus3audio file to open
```
