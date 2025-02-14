# Summary

Prints a "spiral" of numbers starting from the center, outward.

The first number is always one (1).

# Usage

Running using `cargo`:
```sh
cargo run 3
```

Produces:
```
21,22,23,24,25
20,7 ,8 ,9 ,10
19,6 ,1 ,2 ,11
18,5 ,4 ,3 ,12
17,16,15,14,13
```
# Motivation

Wanted to do something easy as a practical example to begin learning Rust.

# TODO

1. Refactor the code to be more idiomatic.
2. Allow the user to specify the starting corner and direction.
3. Terminate at a number other than one (1).
4. Add tests!
