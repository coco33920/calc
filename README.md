# Calc
<div align="center">

**Calc: a minimalistic calculator built in rust for educational purpose only.**

[![Rust Test](https://github.com/coco33920/calc/actions/workflows/rust-test.yml/badge.svg)](https://github.com/coco33920/calc/actions/workflows/rust-test.yml)
[![Release](https://img.shields.io/github/v/release/coco33920/calc.svg?include_prereleases=&sort=semver&color=f7a8d8)](https://github.com/coco33920/calc/releases/latest)
![](https://img.shields.io/crates/v/mini-calc?link=https%3A%2F%2Fcrates.io%2Fcrates%2Fmini-calc)
![](https://img.shields.io/crates/l/mini-calc?link=https%3A%2F%2Fgithub.com%2coco33920%2Fcalc%2Fblob%2Fmaster%2FLICENCE)

</div>

## Install

You can install the latest version from source

```bash 
git clone https://github.com/coco33920/calc
cd calc 
cargo build --release
./target/release/mini-calc
```

or install it via cargo

```bash 
cargo install mini-calc
```

## TODO List

- [X] Lexing of basic operations
    - [X] Lexing operators
    - [X] Lexing lpar,rpar,quote
    - [X] Lexing int
    - [X] Lexing floats
    - [X] Lexing identifiers
- [X] Parsing of basic operations
    - [X] Parsing int,floats,identifiers
    - [X] sum operation
    - [X] minus operation
    - [X] multiplication operation
    - [X] division operation
- [X] Parsing advanced operations
    - [X] Parsing lpar,rpar
    - [X] mathematical priority
      - [X] Left priority
      - [X] Right priority
    - [X] parenthesis support
    - [X] Assignment
    - [X] associativity support
      - [X] Left associativity
      - [X] Right associativity
- [X] Application REPL
    - [X] Add the REPL
        - [X] basic REPL
        - [X] colour message
        - [X] colourised prompt
    - [X] Add colour
- [X] Interpreter
    - [X] Basic operation interpreter
    - [X] Advanced operation interpreter
    - [X] Identifiers (variable) interpreter
- [ ] Config
    - [ ] Config colours
    - [ ] Config prompt
- [ ] Add more operations
  - [ ] exponent
- [ ] Add support for functions
    - [ ] exp
    - [ ] ln
    - [ ] log base a
    - [ ] cos/sin/tan
    - [ ] cosh/sinh/tanh
- [ ] For later
    - [ ] Defining your own functions
    - [ ] Add RPN mode
    - [ ] Hidden multiplication

## Examples

### REPL with only Lexing (verbose mode: on by default)

![](docs/assets/test_lexing.png)

### REPL with lexing and basic operation parsing (verbose mode: on by default)

![](docs/assets/test_parsing_basic_operations.png)

### REPL and functionning interpreter (verbose mode: off by default)

![](docs/assets/test_interpreter.png)

## Configuration