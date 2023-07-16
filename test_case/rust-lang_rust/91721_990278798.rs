plain
    Checking core v0.0.0 (/checkout/library/core)
error[E0433]: failed to resolve: partially resolved path in a macro
  --> library/core/tests/future.rs:91:17
   |
91 |         assert!(Option::is_none!(&try { join!(maybe_fut?, async { unreachable!() }) }));
   |                 ^^^^^^^^^^^^^^^ partially resolved path in a macro

error[E0728]: `await` is only allowed inside `async` functions and blocks
   |
   |
95 |     fn join_is_able_to_handle_temporaries() {
   |        ---------------------------------- this is not `async`
96 |         let _ = join!(async_fn(&String::from("temporary")));
97 |         let () = block_on(join!(async_fn(&String::from("temporary"))).await);
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
error[E0277]: `()` is not a future
   --> library/core/tests/future.rs:97:27
    |
    |
97  |         let () = block_on(join!(async_fn(&String::from("temporary"))).await);
    |                  -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `()` is not a future
    |                  required by a bound introduced by this call
    |
    |
    = help: the trait `std::future::Future` is not implemented for `()`
    = note: () must be a future or must implement `IntoFuture` to be awaited
note: required by a bound in `future::block_on`
    |
    |
101 | fn block_on(fut: impl Future) {
    |                       ^^^^^^ required by this bound in `future::block_on`
Some errors have detailed explanations: E0277, E0433, E0728.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `core` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
