# OxyLisp

A lisp Tree-Walk interpreter written in Rust

Wrote this interpreter mainly for learning rust, the language is inspired by [Fennel](https://fennel-lang.org/).

## Usage

To run the interpreter, simply clone the repository and run with cargo.

```bash
git clone git@github.com:lrangell/OxyLisp.git
cd OxyLisp
cargo run

```


## Data structures

### Lists

Lists can be defined by `[]` and elements are separated by white space

```fennel
[1 2 3]

```

```fennel
(def foo ["bar" "fux"])

```
### Records

Records are defined by key and value pairs.

```fennel
{:a [3 5 7] :b "foo"}

```

## Built-in primitives

### Variable and function definition

Variables are defined by `def` and functions by `defn`. Additionally, you can write anonymous functions using `fn`.

```fennel
(def x 42)

(defn double [n] (* 2 n))

(def plus-five (map (fn [n] (+ n 5)) (range 0 10)))

```

### List manipulation
  fold, map, concat and range are some of the built-in functions
