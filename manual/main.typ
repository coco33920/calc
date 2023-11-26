#import "@preview/codly:0.1.0": *
#set page(numbering: "1/1")
#set align(center)
#set text(font:"Monaspace Xenon")
#set text(size: 18pt,weight: "bold")
Calc Manual\
#set text(size: 13pt,weight: "regular")
November 26th 2023
#set align(left)
#show heading.where(level:1): set align(right)
#set heading(numbering: "I.1.")

#outline(title: [Table of Contents])
#pagebreak(weak:true)

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
#link("https://calc.nwa2coco.fr/book",[*The Online Book*])
