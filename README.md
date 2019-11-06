# Polynom

Simple library for dealing with polynomials in Rust.

To view the generated documentation, run
```
cargo doc --open
```
To run the test suite,
```
cargo test
```

## Examples

A new polynomial can be created from a vector of coefficients and an indeterminate as follows: 
```rust
use polynom::polynomial::Polynomial;

let polynomial = Polynomial::new(vec![1f64, 2f64, 3f64], 'x');

assert_eq!(polynomial.as_string(), String::from("f(x) = 1 + 2x + 3x^2")
```

## Credits
This was inspired by Ch.1, Section 4 of Jeremy Kun's [A Programmer's Introduction to Mathematics](https://pimbook.org). This library is a port of the [`polynomial.py`](https://github.com/pim-book/programmers-introduction-to-mathematics/blob/master/secret-sharing/polynomial.py) class Kun provides in the [GitHub repo](https://github.com/pim-book/programmers-introduction-to-mathematics/) that accompanies the book.

## Note
This library is incomplete and definitely not optimal. you probably shouldn't use it in production.