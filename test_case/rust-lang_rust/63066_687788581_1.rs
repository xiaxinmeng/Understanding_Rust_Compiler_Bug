
error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
   --> src/main.rs:139:37
    |
139 |     client: websocket::sync::Client<impl websocket::sync::Stream>,
    |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
