
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

note: rustc 1.57.0-beta.1 (d4647278c 2021-10-18) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental -C target-cpu=native --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck] type-checking `main`
#1 [typeck_item_bodies] type-checking all item bodies
end of query stack
