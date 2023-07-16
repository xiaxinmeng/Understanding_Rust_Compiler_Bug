terminal
$ cargo +1.56.1 build
   Compiling report-bug v0.1.0 (/home/Toru3/codes/report-bug)
error[E0107]: this function takes 2 generic arguments but 1 generic argument was supplied
  --> src/main.rs:32:13
   |
32 |     let b = add::<i32>(x, y); // ICE
   |             ^^^   --- supplied 1 generic argument
   |             |
   |             expected 2 generic arguments
   |
note: function defined here, with 2 generic parameters: `T`, `U`
  --> src/main.rs:20:4
   |
20 | fn add<T, U>(x: T, y: T) -> T
   |    ^^^ -  -
help: add missing generic argument
   |
32 |     let b = add::<i32, U>(x, y); // ICE
   |                      +++

error[E0275]: overflow evaluating the requirement `for<'x> &'x Point<_>: Add`
  --> src/main.rs:32:13
   |
32 |     let b = add::<i32>(x, y); // ICE
   |             ^^^^^^^^^^
   |
   = help: consider adding a `#![recursion_limit="256"]` attribute to your crate (`report_bug`)
note: required because of the requirements on the impl of `for<'x> Add` for `&'x Point<Point<_>>`
  --> src/main.rs:7:13
   |
7  | impl<'a, T> Add for &'a Point<T>
   |             ^^^     ^^^^^^^^^^^^
   = note: 127 redundant requirements hidden
   = note: required because of the requirements on the impl of `for<'x> Add` for `&'x Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<Point<_>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
note: required by a bound in `add`
  --> src/main.rs:23:20
   |
20 | fn add<T, U>(x: T, y: T) -> T
   |    --- required by a bound in this
...
23 |     for<'x> &'x T: Add<Output = U>,
   |                    ^^^^^^^^^^^^^^^ required by this bound in `add`

Some errors have detailed explanations: E0107, E0275.
For more information about an error, try `rustc --explain E0107`.
error: could not compile `report-bug` due to 2 previous errors
$ cargo +1.57.0 build
   Compiling report-bug v0.1.0 (/home/Toru3/codes/report-bug)
error[E0107]: this function takes 2 generic arguments but 1 generic argument was supplied
  --> src/main.rs:32:13
   |
32 |     let b = add::<i32>(x, y); // ICE
   |             ^^^   --- supplied 1 generic argument
   |             |
   |             expected 2 generic arguments
   |
note: function defined here, with 2 generic parameters: `T`, `U`
  --> src/main.rs:20:4
   |
20 | fn add<T, U>(x: T, y: T) -> T
   |    ^^^ -  -
help: add missing generic argument
   |
32 |     let b = add::<i32, U>(x, y); // ICE
   |                      +++

error: internal compiler error: compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:846:17: ErrorReporting Overflow should not reach `report_selection_err` call

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1146:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0 (f1edd0429 2021-11-29) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental -C target-cpu=native --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
end of query stack
For more information about this error, try `rustc --explain E0107`.
error: could not compile `report-bug` due to previous error
