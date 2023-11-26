#import "@preview/codly:0.1.0": *
#set page(numbering: "1/1")
#set align(center)
#set text(font:"Monaspace Xenon")
#set text(size: 18pt,weight: "bold")
Calc Manual\
#set text(size: 13pt,weight: "regular")
Last updated, November, The 26th, 2023
#set align(left)
#show heading.where(level:1): set align(right)
#set heading(numbering: "I.1.")

#set list(indent: 1cm,body-indent: 0.25cm)

#outline(title: [Table of Contents],indent: auto)
#pagebreak(weak:true)

#let icon(codepoint) = {
  box(
    height: 0.8em,
    baseline: 0.05em,
    image(codepoint)
  )
  h(0.1em)
}

#show: codly-init.with()
#codly(languages: (
  rust: (name: "Rust", icon: icon("brand-rust.svg"), color: rgb("#CE412B")),
  sh: (name: "Bash", icon: icon("brand-bash.svg"), color: rgb("3c3c3c")),
))

#let calc = link("https://calc.nwa2coco.fr",[#set text(red); *Calc*])

= Introduction
#v(1em)
#calc is a fully-featured calculator written in Rust for education purpose, it 
was designed to be minimalistic but then went off the rails and a lot of feature
where implemented.

Now #calc is a powerful calculator capable of exact rational computation,
matrix and vectors algebra, bindings to gnuplot and terminal plotting, with
dozens of updates and currently (as of writing this manual) in version *2.11.4*.

If you prefer a website you may want to read
#link("https://calc.nwa2coco.fr/book",[*The Online Book*]) which is always up to
date.

== Install

You can install it via cargo 

```sh
cargo install mini-calc
```

or via the source

```sh
git clone https://github.com/coco33920/calc
cd calc 
cargo build --release
./target/release/mini-calc
```

Visit #calc to see all the install page

== Contributors 


#table(
  columns: (auto, auto, auto),
  inset: 10pt,
  align: horizon,
  [*Name*], [*Role*], [*Website*],
  "Charlotte THOMAS",
  "Main developer/Maintener",
  link("https://me.nwa2coco.fr",[#set text(red); Personal Page]),
  "Léana 江",
  "Help, cleanup",
  link("https://earth2077.fr",[#set text(red); Website/Blog])
)

#pagebreak(weak: true)

= Usage

== Basic operators

#calc have the basic operators which are

- `+` for the addition
- `-` for the substraction
- `*` for the multiplication
- `/` for the division (or for a rational)
- `^` for the exponentation

== Variables

It also supports variable the syntax is 

```
myvar = value
```

for example

```
var = (2+2)
``` 

#set align(center)
#figure(
  image("assets/image.png",height: 30%, width: auto),
  caption: [Example of setting a variable]
)
#set align(left)

== Built-in variables

The following variables are built-in:

- `pi` is pi as a double precision float
- `e` is e as a double precision float

#pagebreak(weak: true)

= Functions

== Implemented

The following functions are currently implemented:

*Trigonometry* 

- `sin` (vectorized)
- `cos` (vectorized)
- `tan` (vectorized)

*Hyperbolic trigonometry*
- `sinh` (vectorized)
- `cosh` (vectorized)
- `tanh` (vectorized)

*Reverse trigonometry*
- `acos` (vectorized)
- `asin` (vectorized)
- `atan` (vectorized)

*Exponentiation*
- `exp` (vectorized)
- `ln` (alias: log) (vectorized)

*Vectors*
- `norm`

*Matrices*
- `det`
- `invert`

*Plot*
- `plot`
- `termplot`

*Other*
- `sqrt` (vectorized)
- `factorial` (alias: fact)
- `abs`
- `ceil`
- `floor`
- `round`

== Trigonometry 

For trigonometry, the input is assumed to be in radians, if it is in degrees you
need to add `false` or `true` as a second argument, example shown bellow.

#set align(center)
#figure(
  image("assets/trigo.png"),
  caption: [Usage of trigonometry]
)
#set align(left)

#pagebreak(weak: true)
== Exp/ln 

If you use the exp function you can pass as a second argument the base you want
to use if no second arguments are passed it will used the natural base.

#set align(center)
#figure(
  image("assets/expln.png"),
  caption: [Usage of exp/ln]
)
#set align(left)

#pagebreak(weak: true)
== Root 

You can specify in second argument an integer to take the nth root, if not it
take the square root.

#set align(center)
#figure(
  image("assets/nth_root.png"),
  caption: [Usage of sqrt]
)
#set align(left)

== Partial function

The calculator's language supports partial function.

#set align(center)
#figure(
  image("assets/function.png"),
  caption: [Example of a partial function]
)
#set align(left)

#pagebreak(weak: true)
== Vectorization
Functions have been vectorized.

#set align(center)
#figure(
  image("assets/sqrt_vectorized.png"),
  caption: [Example of a vectorized function]
)
#set align(left)


