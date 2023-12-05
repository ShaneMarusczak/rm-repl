# rm-repl [![Rust](https://github.com/ShaneMarusczak/rm-repl/actions/workflows/rust.yml/badge.svg?branch=main&event=push)](https://github.com/ShaneMarusczak/rm-repl/actions/workflows/rust.yml)

A repl (read–eval–print loop) for rusty maths

THIS TOOL IS A WORK IN PROGRESS, FEATURES MAY CHANGE AT ANY TIME WITH NO NOTICE.

<img src="./images/Screenshot%202023-12-05%20at%204.57.16 PM.png" width="768"/>

## Description of features

- Evaluation of mathematical expressions passed in at the command line or in a
  repl session.
- Equation graphing.
- Builds tables of data points for an equation.
- Vector manipulation.

## Usage instructions

- Clone this repository and install as a binary to your path with
  `cargo install --path .` or run with `cargo run`.

- Calling the program directly with no arguments will place you in a repl
  session.

  ```
  rmr
  ```

- In a repl session expressions are evaluated upon pressing enter.
  ```
  >>21+21
  42
  >>
  ```
- In a repl session you can pass commands that begin with a `:` to change modes.
  ```
  >>:g
  equation:
  ```
- Available commands are:

  ```
  :g  | :graph -> graphing mode
  :t  | :table -> table mode
  :go | :graph options -> graph options mode
  :ag | :animated graph -> animated graph mode
  :ig | :interactive graph -> interactive graph mode
  :la | :linear algebra -> linear algebra mode
  :q  | :quit -> exits the repl session
  ```
- In linear algebra mode the following commands become valid:
  ```
  :vs | :vector sum -> vector sum mode
  :vm | :vector mean -> vector mean mode
  ```

- Graph mode accepts multiple equations separated by a `|`. Both equations will
  be plotted in the same graph space adjusting for equations with different
  ranges.
  ```
  >>:g
  equation:y=sin(x) | y=cos(x)
  ```

- Alternatively you can pass flags and arguments to the program directly.
- Passing no flags is interpreted as evaluation mode.

  ```
  rmr 84/2
  42
  ```
- NOTE: '(' and ')' are not allowed to be used directly at the command line. Any
  expression using these must be wrapped in quotes.

  ```
  rmr "sqrt(1764)"
  42
  ```

- Available flags are:
  ```
  -g | --graph -> graphing mode
  rmr -g y=x -5 5

  -t | --table -> table mode
  rmr -t y=x -5 5 1
  ```
