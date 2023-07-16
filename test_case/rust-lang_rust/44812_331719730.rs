
[00:34:22] error[E0523]: found two different crates with name `env_logger` that are not distinguished by differing `-C metadata`. This will result in symbol conflicts between the two.
[00:34:22]   --> /checkout/src/librustdoc/lib.rs:37:1
[00:34:22]    |
[00:34:22] 37 | extern crate rustc_driver;
[00:34:22]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:34:22] 
[00:34:22] error: Could not compile `rustdoc`.
