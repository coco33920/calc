# Calc

<div align="center">

<span style="font-weight:bold">Calc: a fully-featured minimalistic calculator built in rust for educational purpose only.</span>
<br/>
<a href="https://github.com/coco33920/calc/actions/workflows/rust-test.yml"><img src="https://github.com/coco33920/calc/actions/workflows/rust-test.yml/badge.svg"></a>
<a href="https://github.com/coco33920/calc/releases/latest"><img src="https://img.shields.io/github/v/release/coco33920/calc.svg?include_prereleases=&sort=semver&color=f7a8d8"></a>
<a href="https://crates.io/crates/mini-calc"><img src="https://img.shields.io/crates/v/mini-calc?link=https%3A%2F%2Fcrates.io%2Fcrates%2Fmini-calc"></a>
<a href="https://github.com/coco33920/calc/blob/master/LICENSE"><img src="https://img.shields.io/crates/l/mini-calc?link=https%3A%2F%2Fgithub.com%2coco33920%2Fcalc%2Fblob%2Fmaster%2FLICENCE"></a>
<a href="https://crates.io/crates/mini-calc"><img src="https://img.shields.io/crates/d/mini-calc"/></a>

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

You can see how the calculator works over at [the usage page](usage.md)

## Config

An overview of the configuration of mini-calc can be found [here](config.md)

## Function 

An overview of the function of mini-calc can be found [here](function.md) 

## Logic

To learn about the binary logic built in go to the [logic page](logic.md)

## Plot !

You can plot more information in [the plot page](plot.md)

## User defined functions

You can define your own functions!

[![img.png](../assets/user_defined.png)](../assets/user_defined.png)

## Vector calculation 

You can compute using vectors!

- add vectors
- dot product (* operator)
- norm function

[![](../assets/vector.png)](../assets/vector.png)

## Matrices !

As of 2.7.0 matrix algebra is implemented (using lup reduction)

- you can add matrices 
- multiply compatible matrices

functions added
- transpose
- invert
- det 

[![](../assets/matrix.png)](../assets/matrix.png)
