# Rust DynLib in C

Example of using Rust dyn library inside C.

## Usage

Makefile based project, with rust lib files detection. To run just:
```sh
cd core_c/
make run
```
It will:
1. test Rust lib
2. compile rust lib to dyn library
3. attach dyn lib file and run executive
