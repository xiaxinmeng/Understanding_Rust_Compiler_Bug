sh
cargo run
warning: the feature `conservative_impl_trait` has been stable since 1.26.0 and no longer requires an attribute to enable
 --> src/main.rs:1:12
  |
1 | #![feature(conservative_impl_trait)]
  |            ^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(stable_features)]` on by default

warning: trait objects without an explicit `dyn` are deprecated
  --> src/main.rs:19:23
   |
19 |     let http_fut: Box<Future<Item=(),Error=()> + Send> = get().boxed();
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
   = note: `#[warn(bare_trait_objects)]` on by default
help: use `dyn`
   |
19 |     let http_fut: Box<dyn Future<Item=(),Error=()> + Send> = get().boxed();
   |                       +++

warning: unused variable: `http_fut`
  --> src/main.rs:19:9
   |
19 |     let http_fut: Box<Future<Item=(),Error=()> + Send> = get().boxed();
   |         ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_http_fut`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: unnecessary `unsafe` block
 --> src/main.rs:9:9
  |
9 |         unsafe{
  |         ^^^^^^ unnecessary `unsafe` block
  |
  = note: `#[warn(unused_unsafe)]` on by default

warning: unused `Result` that must be used
  --> src/main.rs:11:13
   |
11 |             tx.send(());
   |             ^^^^^^^^^^^
   |
   = note: this `Result` may be an `Err` variant, which should be handled
   = note: `#[warn(unused_must_use)]` on by default
help: use `let _ = ...` to ignore the resulting value
   |
11 |             let _ = tx.send(());
   |             +++++++

warning: `test_43889` (bin "test_43889") generated 5 warnings (run `cargo fix --bin "test_43889"` to apply 2 suggestions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/test_43889`

