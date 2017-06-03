# `comp-rs`

[![Build Status](https://travis-ci.org/goandylok/comp-rs.svg?branch=master)](https://travis-ci.org/goandylok/comp-rs)
[![crates.io](https://img.shields.io/crates/v/comp.svg)](https://crates.io/crates/comp)
[![docs.rs](https://docs.rs/comp/badge.svg)](https://docs.rs/comp)

Pure-macro Do notation and List-comprehension for Option, Result and Iterator.

It provides syntax extensions to easily combind wrapper type (`Option`, `Result` and `Iterator`), which seems like
`for-comprehension` in scala or `Do notation` in haskell.

### [**Documentation**](https://docs.rs/comp/)

## Usage

First, add the following to your `Cargo.toml`:

```toml
[dependencies]
comp = "*"
```

Next, add this to your crate root:

```rust
#[macro_use]
extern crate comp;
```

## Example

`comp-rs` delivers three macros : *`option!`*, *`result!`* and *`iter!`*,
transforming the `arrow(<-)` statements into FP bind (`flat_map`).

### Iterator

```rust
#[macro_use]
extern crate comp;

let iter = iter! {
  let x <- 0..2u8;
  let y <- vec!['a', 'b'];
  (x, y)
};

for x in iter {
  println!("{:?}", x);
}

// Print (0, 'a') (0, 'b') (1, 'a') (1, 'b')
```

You can also have Python like generator expressions:

```Python
# Generator expression in Python:

values = [6, 2, 9, 4, -1, 33, 87, 23]
result = (x*x for x in values if x < 10)
```

```Rust
// "Generator" expression in Rust (also in just one line):

let values = vec![6, 2, 9, 4, -1, 33, 87, 23];
let result = iter!{let x <- values; if x < 10; x*x};
```

Note that the order is reversed:
- first let binding
- then the guard
- and the result expression as last item

The result is of type FlatMap and also lazy, like Pythons generator expressions.


### Option
```rust
#[macro_use]
extern crate comp;

let option = option! {
  let a <- Some(1);
  let b <- Some(2);
  a + b
};

assert_eq!(option, Some(3));
```

### Result

Unlike `Iterator` and `Option`, rust provides __*Question Mark*__ syntax to combine `Result`s.

Let's see how `comp-rs` makes it more explicit and expressive.

#### Native way

```rust
#[macro_use]
extern crate comp;

use std::fs::File;
use std::io;
use std::io::prelude::*;

// try!() macro must be wrap into a function
fn content() -> io::Result<String> {
    let mut f = try!(File::open("foo.txt"));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}
```

#### Question mark

```rust
#[macro_use]
extern crate comp;

use std::fs::File;
use std::io;
use std::io::prelude::*;

// '?' mark must be wrap into a function
fn content() -> io::Result<String> {
    let mut f = File::open("foo.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

#### `result!` way

```rust
#[macro_use]
extern crate comp;

use std::fs::File;
use std::io;
use std::io::prelude::*;

let content: io::Result<String> = result! {
  let mut f <- File::open("foo.txt");
  let mut s = String::new();
  let _ <- f.read_to_string(&mut s);
  s
};
```

## Contribution

All kinds of contribution are welcome.

- **Issue** Feel free to open an issue when you find typos, bugs, or have any question.
- **Pull requests**. Better implementation, more tests, more documents and typo fixes are all welcome.

## License

Licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
