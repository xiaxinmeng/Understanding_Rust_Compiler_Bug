
Compiling playground v0.0.1 (/playground)
error[[E0277]](https://doc.rust-lang.org/nightly/error-index.html#E0277): the type `&mut Context<'_>` may not be safely transferred across an unwind boundary
   --> src/main.rs:5:9
    |
5   | /         async {
6   | |             async {}.await; // await inside a `.catch_unwind()` currently fails
7   | |         }
    | |         ^
    | |         |
    | |_________`&mut Context<'_>` may not be safely transferred across an unwind boundary
    |           within this `[async block@src/main.rs:5:9: 7:10]`
    |
    = help: within `[async block@src/main.rs:5:9: 7:10]`, the trait `UnwindSafe` is not implemented for `&mut Context<'_>`
    = note: `UnwindSafe` is implemented for `&std::task::Context<'_>`, but not for `&mut std::task::Context<'_>`
note: future does not implement `UnwindSafe` as this value is used across an await
   --> src/main.rs:6:21
    |
5   | /         async {
6   | |             async {}.await; // await inside a `.catch_unwind()` currently fails
    | |                     ^^^^^^ await occurs here, with the value maybe used later
7   | |         }
    | |         -
    | |         |
    | |_________the value is later dropped here
    |           has type `&mut Context<'_>` which does not implement `UnwindSafe`
note: required by a bound in `futures::FutureExt::catch_unwind`
   --> /playground/.cargo/registry/src/github.com-1ecc6299db9ec823/futures-util-0.3.25/src/future/future/mod.rs:433:23
    |
433 |         Self: Sized + ::std::panic::UnwindSafe,
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `FutureExt::catch_unwind`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `playground` due to previous error
