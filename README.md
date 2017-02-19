# `comp-rs`

Pure-macro Do notation and List-comprehension for Option, Result and Iterator.

It provides syntax extensions separately for the three types above, which look like
`for-comprehension` in scala or `Do notation` in haskell.

### [**Documentation**](https://goandylok.github.io/arraydeque/doc/arraydeque/index.html)

## Usage

First, add the following to your `Cargo.toml`:

```toml
[dependencies]
comp = "0.1"
```

Next, add this to your crate root:

```rust
#[macro_use]
extern crate comp;
```

## Example

`comp-rs` delivers three macros : *`option!`*, *`result!`* and *`iter!`*,
transforming the `arrow(<-)` statements into FP binding( *`flat_map()`* ).

### Iterator

```rust
#[macro_use]
extern crate comp;

let iter = iter! {
  let x <- 0..2;
  let y <- vec!['a', 'b'];
  (x, y)
}

for x in iter {
  println!("{}", x);
}

// Print (0, 'a') (0, 'b') (1, 'a') (1, 'b')
```

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
use std::fs::File;
use std::io::prelude::*;

let content: Result<String> = {
  let mut f = try!(File::open("foo.txt"));
  let mut s = String::new();
  try!(f.read_to_string(&mut s));
  s
};

```

#### Question mark

```rust
use std::fs::File;
use std::io::prelude::*;

let content: Result<String> = {
  let mut f = File::open("foo.txt")?;
  let mut s = String::new();
  f.read_to_string(&mut s)?;
  s
};
```

#### `comp-rs` way

```rust
#[macro_use]
extern crate comp;

use std::fs::File;
use std::io::prelude::*;

let content: Result<String> = result! {
  let mut f <- File::open("foo.txt");
  let mut s = String::new();
  let size <- f.read_to_string(&mut s);
  s
};
```

## Syntax

All three macros return wrapped type(`Option<T>`, `Result<T>` and
  `Iterator<Item=T>`), and yield the last expression.

Syntax: `(sentence)* ; expression`

sentence can be:
* `let pattern <- expression;`: bind expression to pattern.
* `if filter_expression;`: filter by condition, and jump over when not satisfied.
* `statement;`: let assignment, value assignment, etc.
* `{...}`: block and unsafe block.


## Syntax Detail

### 1. Basic arrow(<-) syntax

#### Rules

```rust
Macro                      Expand
                  |
option! {}        |        Some(())
                  |
option! { x; }    |        { x; Some(()) }
                  |
option! { x }     |        Some(x)
                  |

Macro
------------------------------------          
Expand

option! { let x <- Some(1); }
------------------------------------
Some(1).and_then(move |x| option!{})

option! { let x <- Some(1); x }
------------------------------------
Some(1).and_then(move |x| option!{ x })

option! { let mut x <- Some(1); x }
------------------------------------
Some(1).and_then(move |mut x| option!{ x })
```

#### Example

```rust
let option = option! {
  let a <- Some(1);
  let b <- Some(2);
  a + b
};

// code above is expanded roughly into this

let option = {
  Some(1).and_then(move |a| {
    Some(2).and_then(move |b| {
      Some(a + b)
    })
  })
};
```

```rust
let iter = iter! {
  let x <- 0..2;
  let y <- vec!['a', 'b'];
  (x, y)
}

// code above is expanded roughly into this

let iter = {
  (0..2).into_iter().flat_map(move |x| {
    (vec!['a', 'b']).into_iter().flat_map(move |y| {
      ::std::iter::once((x, y))
    })
  })
};
```

### 2. Yield

The last expression of the block will be yielded, similar to functions in rust.

```rust
let iter = iter! {
  let x <- 0..2;
  let y <- vec!['a', 'b'];

  (x, y)    // <------- Yield
}
```

The block yields `()` while the last line is __*arrow statement*__ or statement
with __*semicolon*__.

```rust
let option: Option<()> = option! {
  let a <- Some(1);
  let b <- Some(2);
};

let option: Option<()> = option! {
  let a <- Some(1);
  let b <- Some(2);
  a + b;
};
```

### 3. Pattern

In `comp-rs`, pattern is supported as it should be.

#### Tuple

```rust
let option = option! {
  let (x, y) <- Some((1, 2));  
  (y, x)
};

assert_eq!(option, Some((2, 1)));
```

#### Struct

```rust
struct Struct { x: usize };

let option = option! {
  let Struct { x } <- Some(Struct { x: 1 });  
  x
};

assert_eq!(option, Some(1));
```

#### Ignore

```rust
let option = option! {
  let _ <- Some(1);
};
```

#### ... And So On

### 4. If-Guard

If-Guard is specific for `iter!` which translates condition into `filter()`.

It wraps the following code into a block and call `filter()` on it.

```rust
let iter = iter! {
  let x <- 0..4;
  let y <- 2..6;

  if x == y;

  // won't reach here if condition isn't satisfied
  (x, y)
};

let expected = vec![(2, 2), (3, 3)];
assert!(expected, iter.collect::<Vec<_>>());
```

### 5. Statement & Block

Statements and blocks are also supported.

```rust
// statement
let iter = iter! {
  let start = 5;
  let end;
  end = start * 3;

  // 5, 6, ..., 13, 14
  let x <- start..end;
  x
};
let expected = 5..15;
assert!(iter.eq(expected.into_iter()));
```
```rust
let iter = iter! {
    let mut a <- 0..5;

    // block
    {
        fn double(x: u8) -> u8 { x * 2}
        let tmp = double(a);
        a = tmp;
    };

    // unsafe block
    let count = unsafe {
        static mut CALL_COUNT: u8 = 0;
        CALL_COUNT += 1;
        CALL_COUNT
    };

    (a, count)
};
let expected = vec![(0, 1), (2, 2), (4, 3), (6, 4), (8, 5)];
assert!(iter.eq(expected.into_iter()));
```

### Array

`Array` in rust behaves differently from other collections. It only iterates its
content by reference.
So `iter!` always binds *references* in `arrow(<-)` syntax, then you need to
*deref* the bound value.
And since one can't move any value out of an `array`, array should be placed
outside the macro to satisfy lifetime.

```rust
let array = [0, 1, 2, 3];
let iter = iter! {
    let x <- array;
    let y <- *x..4;
    (*x, y)
};
let expected = vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 1), (1, 2), (1, 3), (2, 2),
                    (2, 3), (3, 3)];
assert!(expected, iter.collect::<Vec<_>>());
```

## Contribution

All kinds of contribution are welcome.

- **Issue** Feel free to open an issue when you find typos, bugs, or have any question.
- **Pull requests**. Better implementation, more tests, more documents and typo fixes are all welcome.

## License

Licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
