# rm-repl   [![Rust](https://github.com/ShaneMarusczak/rm-repl/actions/workflows/rust.yml/badge.svg?branch=main&event=push)](https://github.com/ShaneMarusczak/rm-repl/actions/workflows/rust.yml)

a simple repl for rusty maths


## installation

clone repository and use `cargo run`

alternatively can be added to your path as an executable binary by using `cargo install --path .`

this will install as `rmr`

## usage

running will drop you into a repl session

by default each line is evaluated as a literal mathematical expression

alternatively, lines prefixed with `:` will be treated as special commands

currently two commands are supported:

`:q` exits the repl and terminates the program

`:p` will enter plotting mode and prompts will guide you through entering an equation
