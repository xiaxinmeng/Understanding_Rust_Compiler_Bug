
   Compiling playground v0.0.1 (/playground)
error[E0119]: conflicting implementations of trait `std::cmp::PartialEq<Op<_>>` for type `Op<_>`:
  --> src/lib.rs:1:10
   |
1  |   #[derive(PartialEq, Eq)]
   |            ^^^^^^^^^ conflicting implementation for `Op<_>`
...
17 | / impl<T, U> PartialEq<Op<T>> for Op<U>
18 | | where
19 | |     T: PartialEq<U>,
20 | | {
...  |
26 | |     }
27 | | }
   | |_- first implementation here
   |
   = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

For more information about this error, try `rustc --explain E0119`.
error: could not compile `playground`.

To learn more, run the command again with --verbose.
