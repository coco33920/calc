# Calc

<div align="center">

**Calc: A fully-featured minimalistic configurable rust calculator**

[![Rust Test](https://github.com/coco33920/calc/actions/workflows/rust-test.yml/badge.svg)](https://github.com/coco33920/calc/actions/workflows/rust-test.yml)
[![Release](https://img.shields.io/github/v/release/coco33920/calc.svg?include_prereleases=&sort=semver&color=f7a8d8)](https://github.com/coco33920/calc/releases/latest)
[![](https://img.shields.io/crates/v/mini-calc?link=https%3A%2F%2Fcrates.io%2Fcrates%2Fmini-calc)](https://crates.io/crates/mini-calc)
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

## Usage

You can see how the calculator works over at [the usage page](docs/usage.md)

## Configuration

The page walking down the configuration of the project is available [here](docs/config.md)

## Functions

Check [the function page](docs/function.md) for more informations.

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
  - [X] Basic operation interpreter
  - [X] Advanced operation interpreter
  - [X] Identifiers (variable) interpreter
- [X] Built-in
  - [X] pi
  - [X] e
- [ ] Config
    - [X] Config colours
    - [X] Config prompt
    - [ ] Config floating point accuracy
- [X] Add more operations
    - [X] exponent
- [X] Add support for functions
    - [X] exp
    - [X] ln
    - [X] log base a
    - [X] cos/sin/tan
    - [X] cosh/sinh/tanh
    - [X] atan/acos/asin
    - [X] some more
- [X] Add logic
    - [X] Add basic true/false
    - [X] Add binary operator
        - [X] or (&&)
        - [X] and (||)
        - [X] `>=`
        - [X] `>`
        - [X] `<=`
        - [X] `<`
        - [X] `==`
    - [X] Add unary operator
        - [X] not (!)
- [X] Vector calculation
    - [X] Add vectors to the datatypes
    - [X] Lex vectors
    - [X] Parse vectors
    - [X] Add vector operations
      - [X] Dot product
      - [X] Vector norm
      - [X] Vector addition
- [X] Matrix calculation
    - [X] Add matrix to the datatypes
    - [X] Lex matrices
    - [X] Parse matrices
    - [X] Add matrices operation
      - [X] Matrix addition
      - [X] Matrix multiplication
      - [X] Calculate the matrix determinant
      - [X] Calculate the reverse matrix
- [ ] Interact with the configuration
    - [ ] Print current config
    - [ ] Reset config to default
    - [ ] Set config
      - [ ] Set main color
      - [ ] Set prompt color
      - [ ] Set prompt 
      - [ ] Set greeting message
      - [ ] Set greeting color 
    - [ ] Reload config

## Evolution

### Ploting

Plot functions (both stdlib, and runtime) would be useful and appreciated.

- [ ] Plot stdlib functions
- [ ] Plot runtime functions
- [ ] Save plot to png/svg/pdf

### Exact math calculator

As this project provides a frontend with a parser we could plug an exact math engine in the backend to transform `calc` into a 
real exact math calculator.
TODO List for the Exact Math Engine

- [ ] Rational calculation
  - [ ] Implement rational numbers operations 
  - [ ] Rational reduction
- [ ] Irrational calculation 
  - [ ] Implement irrational numbers operations
  - [ ] Irrational reduction
- [ ] Literal calculation
  - [ ] Computes literal expression with uninitialized operators
  - [ ] Literal reduction

### CAS

If we ever go to this step, yes, I will remove the "minimalistic" from the description
- [ ] It's hard.

## Examples

### REPL with only Lexing (verbose mode: on by default)

![](docs/assets/test_lexing.png)

### REPL with lexing and basic operation parsing (verbose mode: on by default)

![](docs/assets/test_parsing_basic_operations.png)

### REPL and functionning interpreter (verbose mode: off by default)

![](docs/assets/test_interpreter.png)

## Configuration

You can configure the general color, greeting message, greeting color, prompt and prompt color from the file for example
in (for linux)

```
~/.config/mini-calc/mini-calc.toml
```

Or situated in your operating system config folder.

### What the configuration looks like

The default configuration looks like this

![img.png](docs/assets/img.png)

### Colors

Available colors are

- purple
- cyan
- blue
- black
- red
- yellow
- green
- white
- an hexadecimal colour (ex: "#f7a8d8")

Default colour (if the configuration fail to load) is Cyan.

### Example of a modified configuration

Configuration:

![img.png](docs/assets/config_modified.png)

It looks like:

![img.png](docs/assets/config_looks.png)

## Functions

The following functions are available

- sin
- cos
- tan
- sinh
- cosh
- tanh
- asin
- acos
- atan
- exp
- ln (alias: log)
- sqrt
- factorial (alias: fact)
- abs
- ceil
- floor
- round

### Trigonometry

For trigonometry, the input are assumed to be in radian, if not, you have to put "false" or "true" as second argument,
example shown bellow

![img.png](docs/assets/trigo.png)

### Exp/ln

If you use the exp function you can pass a second argument for the base you are using, if no second arguments are passed
this is assumed to be in natural base

![img.png](docs/assets/expln.png)

### Root

You can take the nth root with the sqrt function, by default it takes the second root.

![img.png](docs/assets/nth_root.png)

### Round

You can round to the nth decimal with the round function, by default it round to the integer (floor)

![img.png](docs/assets/round.png)

## Logic

You can now use logic! I implemented the following functions:

- or (alias : ||)
- and (alias : &&)
- geq (alias : `>=`)
- gt (alias : `>`)
- leq (alias : `<=`)
- lt (alias :`<`)
- eq (alias : `==`)

Example:

![img.png](docs/assets/logic.png)

## User defined functions!

You can define your own functions!

![img.png](docs/assets/user_defined.png)

## Vector calculation !

You can use vectors! 

- add vector to each others
- added the `norm` function to compute the norm
- dot product between two vectors ( * operator)

Functions added: 
- norm

![](docs/assets/vector.png)

## Matrices !

As of 2.7.0 matrix algebra is implemented (using lup reduction)

- you can add matrices 
- multiply compatible matrices

functions added
- transpose
- invert
- det 

![](docs/assets/matrix.png)
