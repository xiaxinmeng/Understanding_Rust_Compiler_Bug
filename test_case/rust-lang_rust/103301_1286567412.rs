
error[E0308]: mismatched types
    --> src/main.rs:130:12
     |
130  | ...                   .or(interfaces::ossi::lid_handler(lid, parameters))
     |                        -- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `Result`, found opaque type
     |                        |
     |                        arguments to this function are incorrect
     |
note: while checking the return type of the `async fn`
    --> src/interfaces/ossi.rs:621:6
     |
621  | ) -> Result<StatusCode, Rejection> {
     |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ checked the `Output` of this `async fn`, found opaque type
     = note:     expected enum `Result<StatusCode, _>`
             found opaque type `impl Future<Output = Result<StatusCode, Rejection>>`
note: associated function defined here
    --> /home/antoine/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1409:18
     |
1409 |     pub const fn or<F>(self, res: Result<T, F>) -> Result<T, F>
     |                  ^^
help: consider `await`ing on the `Future`
     |
130  |                             .or(interfaces::ossi::lid_handler(lid, parameters).await)
     |                                                                               ++++++
help: try wrapping the expression in `Err`
     |
130  |                             .or(Err(interfaces::ossi::lid_handler(lid, parameters)))
     |                                 ++++                                              +

