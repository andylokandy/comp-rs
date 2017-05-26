#![cfg_attr(not(test), no_std)]

//! Pure-macro Do notation and List-comprehension for Option, Result and Iterator.
//!
//! It provides syntax extensions to easily combind wrapper type (`Option`, `Result` and `Iterator`), 
//! which seems like `for-comprehension` in scala or `Do notation` in haskell.
//!
//! # Usage
//!
//! First, add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! comp = "0.1"
//! ```
//!
//! Next, add this to your crate root:
//!
//! ```
//! #[macro_use]
//! extern crate comp;
//! # fn main() {}
//! ```
//!
//! # Example
//!
//! `comp-rs` delivers three macros : *`option!`*, *`result!`* and *`iter!`*,
//! transforming the `arrow(<-)` statements into FP binding( *`flat_map()`* ).
//!
//! ## Iterator
//!
//! ```
//! #[macro_use]
//! extern crate comp;
//!
//! # fn main() {
//! let iter = iter! {
//!   let x <- 0..2u8;
//!   let y <- vec!['a', 'b'];
//!   (x, y)
//! };
//!
//! for x in iter {
//!   println!("{:?}", x);
//! }
//!
//! // Print (0, 'a') (0, 'b') (1, 'a') (1, 'b')
//! # }
//! ```
//!
//! ## Option
//! ```
//! #[macro_use]
//! extern crate comp;
//!
//! # fn main() {
//! let option = option! {
//!   let a <- Some(1);
//!   let b <- Some(2);
//!   a + b
//! };
//!
//! assert_eq!(option, Some(3));
//! # }
//! ```
//!
//! ## Result
//!
//! Unlike `Iterator` and `Option`, rust provides __*Question Mark*__ syntax to combine `Result`s.
//!
//! Let's see how `comp-rs` makes it more explicit and expressive.
//!
//! **Native way**
//!
//! ```no_run
//! use std::fs::File;
//! use std::io;
//! use std::io::prelude::*;
//!
//! # fn main() {
//! // try!() macro must be wrap into a function
//! fn content() -> io::Result<String> {
//!     let mut f = try!(File::open("foo.txt"));
//!     let mut s = String::new();
//!     try!(f.read_to_string(&mut s));
//!     Ok(s)
//! }
//! # }
//! ```
//!
//! **Question mark**
//!
//! ```no_run
//! use std::fs::File;
//! use std::io;
//! use std::io::prelude::*;
//!
//! # fn main() {
//! // '?' mark must be wrap into a function
//! fn content() -> io::Result<String> {
//!     let mut f = File::open("foo.txt")?;
//!     let mut s = String::new();
//!     f.read_to_string(&mut s)?;
//!     Ok(s)
//! }
//! # }
//! ```
//!
//! **`result!` way**
//!
//! ```no_run
//! # #[macro_use]
//! # extern crate comp;
//! #
//! use std::fs::File;
//! use std::io;
//! use std::io::prelude::*;
//!
//! # fn main() {
//! let content: io::Result<String> = result! {
//!   let mut f <- File::open("foo.txt");
//!   let mut s = String::new();
//!   let _ <- f.read_to_string(&mut s);
//!   s
//! };
//! # }
//! ```
//!
//! # Syntax
//!
//! All three macros return wrapped type(`Option<T>`, `Result<T>` and
//!   `Iterator<Item=T>`), and yield the last expression.
//!
//! Syntax: `(sentence)* ; expression`
//!
//! sentence can be:

//! * `let pattern <- expression;`: bind expression to pattern.
//!
//! * `if filter_expression;`: filter by condition, and jump over when not satisfied.
//!
//! * `statement;`: let assignment, value assignment, etc.
//!
//! * `{...}`: block and unsafe block.
//!
//! # Syntax Detail
//!
//! ## 1. Basic arrow(<-) syntax
//!
//! ## Rules
//!
//! ```rust,ignore
//! Macro                      Expand
//!                   |
//! option! {}        |        Some(())
//!                   |
//! option! { x; }    |        { x; Some(()) }
//!                   |
//! option! { x }     |        Some(x)
//!                   |
//!
//! Macro
//! ------------------------------------
//! Expand
//!
//! option! { let x <- Some(1); }
//! ------------------------------------
//! Some(1).and_then(move |x| option!{})
//!
//! option! { let x <- Some(1); x }
//! ------------------------------------
//! Some(1).and_then(move |x| option!{ x })
//!
//! option! { let mut x <- Some(1); x }
//! ------------------------------------
//! Some(1).and_then(move |mut x| option!{ x })
//! ```
//!
//! ## Example
//!
//! ```
//! # #[macro_use]
//! # extern crate comp;
//! #
//! # fn main() {
//! let option = option! {
//!   let a <- Some(1);
//!   let b <- Some(2);
//!   a + b
//! };
//!
//! // code above is expanded roughly into this
//!
//! let option = {
//!   Some(1).and_then(move |a| {
//!     Some(2).and_then(move |b| {
//!       Some(a + b)
//!     })
//!   })
//! };
//! # }
//! ```
//!
//! ```
//! # #[macro_use]
//! # extern crate comp;
//! #
//! # fn main() {
//! let iter = iter! {
//!   let x <- 0..2;
//!   let y <- vec!['a', 'b'];
//!   (x, y)
//! };
//!
//! // code above is expanded roughly into this
//!
//! let iter = {
//!   (0..2).into_iter().flat_map(move |x| {
//!     (vec!['a', 'b']).into_iter().flat_map(move |y| {
//!       ::std::iter::once((x, y))
//!     })
//!   })
//! };
//! # }
//! ```
//!
//! ## 2. Yield
//!
//! The last expression of the block will be yielded, similar to functions in rust.
//!
//! ```
//! # #[macro_use]
//! # extern crate comp;
//! #
//! # fn main() {
//! let iter = iter! {
//!   let x <- 0..2;
//!   let y <- vec!['a', 'b'];
//!
//!   (x, y)    // <------- Yield
//! };
//! # }
//! ```
//!
//! The block yields `()` while the last line is __*arrow statement*__ or statement
//! with __*semicolon*__.
//!
//! ```
//! # #[macro_use]
//! # extern crate comp;
//! #
//! # fn main() {
//! let option: Option<()> = option! {
//!   let a <- Some(1);
//!   let b <- Some(2);
//! };
//!
//! let option: Option<()> = option! {
//!   let a <- Some(1);
//!   let b <- Some(2);
//!   a + b;
//! };
//! # }
//! ```
//!
//! ## 3. Pattern
//!
//! In `comp-rs`, pattern is supported as it should be.
//!
//! ## Tuple
//!
//! ```
//! # #[macro_use]
//! # extern crate comp;
//! #
//! # fn main() {
//! let option = option! {
//!   let (x, y) <- Some((1, 2));
//!   (y, x)
//! };
//!
//! assert_eq!(option, Some((2, 1)));
//! # }
//! ```
//!
//! ## Struct
//!
//! ```
//! # #[macro_use]
//! # extern crate comp;
//! #
//! # fn main() {
//! struct Struct { x: usize };
//!
//! let option = option! {
//!   let Struct { x } <- Some(Struct { x: 1 });
//!   x
//! };
//!
//! assert_eq!(option, Some(1));
//! # }
//! ```
//!
//! ## Ignore
//!
//! ```
//! # #[macro_use]
//! # extern crate comp;
//! #
//! # fn main() {
//! let option = option! {
//!   let _ <- Some(1);
//! };
//! # }
//! ```
//!
//! ## 4. If-Guard
//!
//! If-Guard is specific for `iter!` which translates condition into `filter()`.
//!
//! It wraps the following code into a block and call `filter()` on it.
//!
//! ```
//! # #[macro_use]
//! # extern crate comp;
//! #
//! # fn main() {
//! let iter = iter! {
//!   let x <- 0..4;
//!   let y <- 2..6;
//!
//!   if x == y;
//!
//!   // won't reach here if condition isn't satisfied
//!   (x, y)
//! };
//!
//! let expected = vec![(2, 2), (3, 3)];
//! assert_eq!(expected, iter.collect::<Vec<_>>());
//! # }
//! ```
//!
//! ## 5. Statement & Block
//!
//! Statements and blocks are also supported.
//!
//! ```
//! # #[macro_use]
//! # extern crate comp;
//! #
//! # fn main() {
//! // statement
//! let iter = iter! {
//!   let start = 5;
//!   let end;
//!   end = start * 3;
//!
//!   // 5, 6, ..., 13, 14
//!   let x <- start..end;
//!   x
//! };
//! let expected = 5..15;
//! assert!(iter.eq(expected.into_iter()));
//! # }
//! ```
//! ```
//! # #[macro_use]
//! # extern crate comp;
//! #
//! # fn main() {
//! let iter = iter! {
//!     let mut a <- 0..5;
//!
//!     // block
//!     {
//!         fn double(x: u8) -> u8 { x * 2}
//!         let tmp = double(a);
//!         a = tmp;
//!     };
//!
//!     // unsafe block
//!     let count = unsafe {
//!         static mut CALL_COUNT: u8 = 0;
//!         CALL_COUNT += 1;
//!         CALL_COUNT
//!     };
//!
//!     (a, count)
//! };
//! let expected = vec![(0, 1), (2, 2), (4, 3), (6, 4), (8, 5)];
//! assert!(iter.eq(expected.into_iter()));
//! # }
//! ```
//!
//! # Array
//!
//! `Array` in rust behaves differently from other collections. It only iterates its
//! content by reference.
//! So `iter!` always binds *references* in `arrow(<-)` syntax, then you need to
//! *deref* the bound value.
//! And since one can't move any value out of an `array`, array should be placed
//! outside the macro to satisfy lifetime.
//!
//! ```
//! # #[macro_use]
//! # extern crate comp;
//! #
//! # fn main() {
//! let array = [0, 1, 2, 3];
//! let iter = iter! {
//!     let x <- array;
//!     let y <- *x..4;
//!     (*x, y)
//! };
//! let expected = vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 1), (1, 2), (1, 3), (2, 2),
//!                     (2, 3), (3, 3)];
//! assert_eq!(expected, iter.collect::<Vec<_>>());
//! # }
//! ```
//!
//! # Contribution
//!
//! All kinds of contribution are welcome.
//!
//! - **Issue.** Feel free to open an issue when you find typos, bugs, or have any question.
//! - **Pull requests**. Better implementation, more tests, more documents and typo fixes are all welcome.
//!
//! # License
//!
//! Licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

/// syntax extension specific for Option
///
/// See the module-level documentation for more details.
#[macro_export]
macro_rules! option {
    (@as_pat $p: pat) => ($p);

    () => {
        Some(())
    };

    (
        let mut $p: tt <- $e: expr ; $( $t: tt )*
    ) => (
        $e.and_then(move | option! (@as_pat mut $p) | { option! { $( $t )* } } )
    );

    (
        let mut $p: ident : $ty: tt <- $e: expr ; $( $t: tt )*
    ) => (
        $e.and_then(move | mut $p : $ty | { option! { $( $t )* } } )
    );

    (
        let $p: tt <- $e: expr ; $( $t: tt )*
    ) => (
        $e.and_then(move | option! (@as_pat $p) | { option! { $( $t )* } } )
    );

    (
        let $p: tt ( $( $para: tt )* ) <- $e: expr ; $( $t: tt )*
    ) => (
        $e.and_then(move | option! (@as_pat $p ( $( $para )* ) ) | { option! { $( $t )* } } )
    );

    (
        let $p: tt { $( $para: tt )* } <- $e: expr ; $( $t: tt )*
    ) => (
        $e.and_then(move | option! (@as_pat $p { $( $para )* } ) | { option! { $( $t )* } } )
    );

    (
        let $p: ident : $ty: tt <- $e: expr ; $( $t: tt )*
    ) => (
        $e.and_then(move | $p : $ty | { option! { $( $t )* } } )
    );

    (
        $stmt: stmt ; $( $t: tt )*
    ) => (
        { $stmt ; option! { $( $t )* } }
    );

    (
        $e: expr ; $( $t: tt )*
    ) => (
        { $e ; option! { $( $t )* } }
    );

    (
        $e: expr
    ) => (
        Some($e)
    );

    (
        $b: block ; $( $t: tt )*
    ) => (
        $b ; option! { $( $t )* }
    );
}

/// syntax extension specific for Result
///
/// See the module-level documentation for more details.
#[macro_export]
macro_rules! result {
    (@as_pat $p: pat) => ($p);

    () => {
        Ok(())
    };

    (
        let mut $p: tt <- $e: expr ; $( $t: tt )*
    ) => (
        $e.and_then(move | result! (@as_pat mut $p) | { result! { $( $t )* } } )
    );

    (
        let mut $p: ident : $ty: tt <- $e: expr ; $( $t: tt )*
    ) => (
        $e.and_then(move | mut $p : $ty | { result! { $( $t )* } } )
    );

    (
        let $p: tt <- $e: expr ; $( $t: tt )*
    ) => (
        $e.and_then(move | result! (@as_pat $p) | { result! { $( $t )* } } )
    );

    (
        let $p: tt ( $( $para: tt )* ) <- $e: expr ; $( $t: tt )*
    ) => (
        $e.and_then(move | result! (@as_pat $p ( $( $para )* ) )  | { result! { $( $t )* } } )
    );

    (
        let $p: tt { $( $para: tt )* } <- $e: expr ; $( $t: tt )*
    ) => (
        $e.and_then(move | result! (@as_pat $p { $( $para )* } ) | { result! { $( $t )* } } )
    );

    (
        let $p: ident : $ty: tt <- $e: expr ; $( $t: tt )*
    ) => (
        $e.and_then(move | $p : $ty | { result! { $( $t )* } } )
    );

    (
        $stmt: stmt ; $( $t: tt )*
    ) => (
        { $stmt ; result! { $( $t )* } }
    );

    (
        $e: expr ; $( $t: tt )*
    ) => (
        { $e ; result! { $( $t )* } }
    );

    (
        $e: expr
    ) => (
        Ok($e)
    );

    (
        $b: block ; $( $t: tt )*
    ) => (
        $b ; result! { $( $t )* }
    );
}

/// syntax extension specific for Iterator
///
/// See the module-level documentation for more details.
#[macro_export]
macro_rules! iter {
    (@as_pat $p: pat) => ($p);

    () => {
        Some(())
    };

    (
        let mut $p: tt <- $e: expr ; $( $t: tt )*
    ) => (
        $e.into_iter().flat_map(move | iter! (@as_pat mut $p) | { iter! { $( $t )* } } )
    );

    (
        let mut $p: ident : $ty: tt <- $e: expr ; $( $t: tt )*
    ) => (
        $e.into_iter().flat_map(move | mut $p : $ty | { iter! { $( $t )* } } )
    );

    (
        let $p: tt <- $e: expr ; $( $t: tt )*
    ) => (
        $e.into_iter().flat_map(move | iter! (@as_pat $p) | { iter! { $( $t )* } } )
    );

    (
        let $p: tt ( $( $para: tt )* ) <- $e: expr ; $( $t: tt )*
    ) => (
        $e.into_iter().flat_map(move | iter! (@as_pat $p ( $( $para )* ) ) | { iter! { $( $t )* } } )
    );

    (
        let $p: tt { $( $para: tt )* } <- $e: expr; $( $t: tt )*
    ) => (
        $e.into_iter().flat_map(move | iter! (@as_pat $p { $( $para )* } ) | { iter! { $( $t )* } } )
    );

    (
        let $p: ident : $ty: tt <- $e: expr ; $( $t: tt )*
    ) => (
        $e.into_iter().flat_map(move | $p : $ty | { iter! { $( $t )* } } )
    );

    (
        if $e: expr ; $( $t: tt )*
    ) => (
        ( iter! { $( $t )* } ).into_iter().filter(move |_| $e)
    );

    (
        $stmt: stmt ; $( $t: tt )*
    ) => (
        { $stmt ; iter! { $( $t )* } }
    );

    (
        $e: expr ; $( $t: tt )*
    ) => (
        { $e ; iter! { $( $t )* } }
    );

    (
        $e: expr
    ) => (
        Some($e)
    );

    (
        $b: block ; $( $t: tt )*
    ) => (
        $b ; iter! { $( $t )* }
    );    
}

#[cfg(test)]
mod tests {
    #![allow(unused_variables)]
    #![allow(dead_code)]

    fn ok<T>(t: T) -> Result<T, ()> {
        Ok(t)
    }

    #[test]
    fn test_basic() {
        let option = option! {
            let a <- Some(1);
            let b <- Some(2);
        };
        assert_eq!(option, Some(()));

        let option = option! {
            let a <- Some(1);
            let b <- Some('a');
            (a, b)
        };
        assert_eq!(option, Some((1, 'a')));

        let option = option! {
            let a <- Some(1);
            let b <- None::<()>;
            (a, b)
        };
        assert_eq!(option, None);

        let option = option! {
            let a <- None::<()>;
            let b <- Some(2);
            (a, b)
        };
        assert_eq!(option, None);

        let result = result! {
            let a <- ok(1);
            let b <- ok(2);
        };
        assert_eq!(result, Ok(()));

        let result = result! {
            let a <- ok(1);
            let b <- ok('a');
            (a, b)
        };
        assert_eq!(result, Ok((1, 'a')));

        let result = result! {
            let a <- Err::<(), _>(1);
            let b <- Ok('a');
            (a, b)
        };
        assert_eq!(result, Err(1));

        let result = result! {
            let a <- Ok('a');
            let b <- Err::<(), _>(2);
            (a, b)
        };
        assert_eq!(result, Err(2));

        let iter = iter! {
            let x <- vec![0, 1, 2, 3];
            let y <- x..4;
            (x, y)
        };
        let expected = vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 1), (1, 2), (1, 3), (2, 2),
                            (2, 3), (3, 3)];
        assert!(iter.eq(expected.into_iter()));
    }

    #[test]
    fn test_array() {
        let array = [0, 1, 2, 3];
        let iter = iter! {
            let x <- array;
            let y <- *x..4;
            (*x, y)
        };
        let expected = vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 1), (1, 2), (1, 3), (2, 2),
                            (2, 3), (3, 3)];
        assert!(iter.eq(expected.into_iter()));
    }

    #[test]
    fn test_guard() {
        let iter = iter! {
            let x <- 0..4;
            let y <- x..4;
            if x * 2 == y;
            (x, y)
        };
        let expected = vec![(0, 0), (1, 2)];
        assert!(iter.eq(expected.into_iter()));
    }

    #[test]
    fn test_pattern() {
        struct TupleStruct0();
        struct TupleStruct(usize);
        struct TupleStruct2(usize, usize);
        struct Struct {
            x: usize,
        };
        struct Struct2 {
            x: usize,
            y: usize,
        };

        let option = option! {
            let (x, y, z) <- Some((1, 2, 3));
            (x, y, z)
        };
        assert_eq!(option, Some((1, 2, 3)));

        let option = option! {
            let (x, (y, z)) <- Some((1, (2, 3)));
            (x, y, z)
        };
        assert_eq!(option, Some((1, 2, 3)));

        let option = option! {
            let TupleStruct0() <- Some(TupleStruct0());
        };
        assert_eq!(option, Some(()));

        let option = option! {
            let TupleStruct0() <- Some(TupleStruct0());
        };
        assert_eq!(option, Some(()));

        let option = option! {
            let TupleStruct(x) <- Some(TupleStruct(9));
            x
        };
        assert_eq!(option, Some(9));

        let option = option! {
            let TupleStruct2(x, y) <- Some(TupleStruct2(9, 10));
            (x, y)
        };
        assert_eq!(option, Some((9, 10)));

        let option = option! {
            let Struct { x } <- Some(Struct { x: 8 });
            x
        };
        assert_eq!(option, Some(8));

        let option = option! {
            let Struct2 { x, y } <- Some(Struct2 { x: 9, y: 10 });
            (x, y)
        };
        assert_eq!(option, Some((9, 10)));
    }

    #[test]
    fn test_ignore() {
        struct TupleStruct(usize);
        struct TupleStruct2(usize, usize);
        struct Struct {
            x: usize,
        };
        struct Struct2 {
            x: usize,
            y: usize,
        };

        let option = option! {
            let _ <- Some(0);
        };
        assert_eq!(option, Some(()));

        let iter = iter! {
            let _ <- 0..10;
            1
        };
        assert_eq!(iter.sum::<u8>(), 10);

        let option = option! {
            let (_, y, _) <- Some((1, 2, 3));
            y
        };
        assert_eq!(option, Some(2));

        let option = option! {
            let (_, (y, _)) <- Some((1, (2, 3)));
            y
        };
        assert_eq!(option, Some(2));

        let option = option! {
            let TupleStruct(_) <- Some(TupleStruct(9));
        };
        assert_eq!(option, Some(()));

        let option = option! {
            let TupleStruct2(x, _) <- Some(TupleStruct2(9, 10));
            x
        };
        assert_eq!(option, Some(9));

        let option = option! {
            let Struct { x: _ } <- Some(Struct { x: 8 });
        };
        assert_eq!(option, Some(()));

        let option = option! {
            let Struct2 { x, y: _ } <- Some(Struct2 { x: 9, y: 10 });
            x
        };
        assert_eq!(option, Some(9));
    }

    #[test]
    fn test_if_expression() {
        let iter = iter! {
            let x <- 0..5;
            let y <- if x % 2 == 0 { Some(x + 1) } else { None };
            y
        };
        let expected = vec![1, 3, 5];
        assert!(iter.eq(expected.into_iter()));

        let iter = iter! {
            let x <- 0..5;
            if x < 2 { 0 } else { 1 }
        };
        let expected = vec![0, 0, 1, 1, 1];
        assert!(iter.eq(expected.into_iter()));
    }

    #[test]
    fn test_statement() {
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
    }

    #[test]
    fn test_block() {
        let iter = iter! {
            let mut a <- 0..5;

            {
                fn double(x: u8) -> u8 { x * 2}
                let tmp = double(a);
                a = tmp;
            };

            let count = unsafe {
                static mut CALL_COUNT: u8 = 0;
                CALL_COUNT += 1;

                CALL_COUNT
            };

            (a, count)
        };
        let expected = vec![(0, 1), (2, 2), (4, 3), (6, 4), (8, 5)];
        assert!(iter.eq(expected.into_iter()));
    }

    #[test]
    fn test_mut() {
        struct TupleStruct2(usize, usize);

        let option = option! {
            let mut a <- Some(2);
            a = a + 10;

            let (mut b,) <- Some((3,));
            b = b + 10;

            (a, b)
        };
        assert_eq!(option, Some((12, 13)));

        let result = result! {
            let mut a <- ok(2);
            a = a + 10;

            let TupleStruct2(mut b, _) <- ok(TupleStruct2(3, 4));
            b = b + 10;

            (a, b)
        };
        assert_eq!(result, ok((12, 13)));

        let iter = iter! {
            let mut a <- 2..3;
            a = a + 10;
            a
        };
        let expected = vec![12];
        assert!(iter.eq(expected.into_iter()));
    }

    #[test]
    fn test_comments() {
        option! {
            // single line comments
            let a <- Some(1);

            /*
             * block comments
             */
            let b <- Some(2);
        };
    }

    #[test]
    fn test_nested() {
        let iter = iter! {
            let a <- 0..2;

            option! {
                let b <- Some(a);
                (b,)
            }
        };
        let expected = vec![Some((0,)), Some((1,))];
        assert!(iter.eq(expected.into_iter()));
    }
}
