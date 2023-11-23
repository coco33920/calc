# Version 2.10.0 : Add function vectorization

## Function
Add vectorization to 
- cos
- sin 
- tan 
- cosh
- sinh
- tanh
- acos
- asin
- atan
- exp
- ln/log 
- sqrt

# Version 2.9.10 : Fix bug, vector plotting

## Plot 
- add vector plotting

## Terminal plot
- fix edge case by changing the coefficient a little bit 
- add vector termploting

# Version 2.9.9 : Terminal plot

- fix bug if yscale or xscale is 0

# Version 2.9.8

- Test

# Version 2.9.7

- Adds CHANGELOG

# Version 2.9.6

- Adds supports for dist-upgrade

# Version 2.9.4 

## Terminal plot
- fix bug while updating ymax and ymin 

# Version 2.9.3 

## Terminal plot
- fix bug with the x axis row

# Version 2.9.2 

## Terminal plot

See #25 
- fix f(x) = x bug
- add auto scaling on y axis when termplot
- add labels on y axis

# Version 2.9.1 

## Terminal plot

- [X] Add terminal ploting
    - [X] Calculate ploting height and width
    - [X] Computes individual points
    - [X] Generate each line
    - [X] Prints each line

# Version 2.9.0

## Plot!

- fix bug when re-defining functions
- added plot
- added string 
- Plot functions (both stdlib, and runtime) would be useful and appreciated.
Plotting is powered by gnuplot, it will works on Unix-like (MacOS, Linux) but I don't have any idea about Windows

- [X] Plot stdlib functions
- [X] Plot runtime functions
- [X] Save plot to png/svg/pdf

# Version 2.8.1 

- Auto-completion

# Version 2.8.0 

## Interactive config !

- [x] Interact with the configuration
    - [X] Print current config
    - [X] Reset config to default
    - [x] Set config
      - [x] Set main color
      - [x] Set prompt color
      - [x] Set prompt 
      - [x] Set greeting message
      - [x] Set greeting color 
    - [X] Reload config

# Version 2.7.0

## Matrix algebra

- [X] Matrix calculation
    - [X] Add matrix to the datatypes
    - [X] Lex matrices
    - [X] Parse matrices
    - [X] Add matrices operation
      - [X] Matrix addition
      - [X] Matrix multiplication
      - [X] Calculate the matrix determinant
      - [X] Calculate the reverse matrix

# Version 2.6.0

## Vector algebra 

- [X] Vector calculation
    - [X] Add vectors to the datatypes
    - [X] Lex vectors
    - [X] Parse vectors
    - [X] Add vector operations
      - [X] Dot product
      - [X] Vector norm
      - [X] Vector addition

# Version 2.5.1 

- fix error 

# Version 2.5.0 

## User defined function

You can define your own function

[![](docs/assets/user_defined.png)](docs/assets/user_defined.png)

# Version 2.4.0

## Logic done

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

# Version 2.3.1 

## More functions
- sqrt
- factorial (aliases: fact or !)
- abs
- ceil
- floor
- round

# Version 2.3.0

## Functions!

- [X] Add support for functions
    - [X] exp
    - [X] ln
    - [X] log base a
    - [X] cos/sin/tan
    - [X] cosh/sinh/tanh
    - [X] atan/acos/asin

# Version 2.2.2 

Configuration update!

# Version 2.1.1 

bug fix
- reverse operation doing math

# Version 2.1.0

Builts in!
- pi
- e

# Version 2.0.0

New parser!
No edge case

# Version 1.0.0

Functionning interpreter

# Version 0.3.0-alpha

Parsing of advanced operations

# Version 0.2.0-alpha

Parsing of basic operation

# Version 0.1.0-alpha

Lexing and REPL!
