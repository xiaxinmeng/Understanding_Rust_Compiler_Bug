rs
warning: the feature `generic_associated_types` is incomplete and may not be safe to use and/or cause compiler crashes
 --> src/main.rs:1:12
  |
1 | #![feature(generic_associated_types)]
  |            ^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(incomplete_features)]` on by default
  = note: see issue #44265 <https://github.com/rust-lang/rust/issues/44265> for more information

warning: unused variable: `result`
  --> src/main.rs:18:9
   |
18 |     let result: PhantomData<S::Associated<'a>> = PhantomData;
   |         ^^^^^^ help: if this is intentional, prefix it with an underscore: `_result`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: function is never used: `future`
  --> src/main.rs:10:4
   |
10 | fn future<'a, S: Trait + 'a, F>(f: F) -> F
   |    ^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: function is never used: `f`
  --> src/main.rs:17:10
   |
17 | async fn f<'a, S: Trait + 'a>() {
   |          ^

warning: function is never used: `foo`
  --> src/main.rs:22:4
   |
22 | fn foo<'a, S: Trait + 'a>() {
   |    ^^^

warning: unused implementer of `Future` that must be used
  --> src/main.rs:23:5
   |
23 |     future::<'a, S, _>(f::<'a, S>());
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_must_use)]` on by default
   = note: futures do nothing unless you `.await` or poll them

warning: 6 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
