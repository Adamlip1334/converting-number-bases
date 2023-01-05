# Base Converter
A command-line program for converting numbers between any two bases between 2 and 64.

    Rust 1.47 or later
    
Installation

Clone the repository and build the program using cargo build:

```
https://github.com/Adamlip1334/converting-number-bases.git
cd base-converter
cargo build
```
Usage
```
base-converter --input-base <base> --output-base <base> <number>
```
Options
```
-h, --help          Prints help information
-i, --input-base    The base of the input number (required)
-o, --output-base   The base of the output number (required)
```
Examples

Convert a binary number to hexadecimal:
```
base-converter --input-base 2 --output-base 16 1010
```
Output:
```
1010 (base 2) = A (base 16)
```
Convert a hexadecimal number to decimal:
```
base-converter --input-base 16 --output-base 10 FF
```
Output:
```
FF (base 16) = 255 (base 10)
```
Limitations

* Can only handle positive integers.
* Can only handle input bases between 2 and 64.
* Can only handle output bases between 2 and 64.
