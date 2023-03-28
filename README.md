# xorciph
An XOR cipher CLI written in Rust.

Based on https://github.com/cr-0w/xorcrypt

## Usage
```
xorciph [OPTIONS] -k <KEY> -f <FILE>
xorciph [OPTIONS] -k <KEY> -p

Options:
  -k, --key <KEY>    The key used for the xor cipher
  -p, --pipe         Provide shellcode from stdin
  -f, --file <FILE>  Provide shellcode from a file
  -r, --raw          Don't format output
  -h, --help         Print help
  -V, --version      Print version
```
