
[01:09:29] error[E0523]: found two different crates with name `rustc_serialize` that are not distinguished by differing `-C metadata`. This will result in symbol conflicts between the two.
[01:09:29]   --> /checkout/src/tools/rls/src/build.rs:13:1
[01:09:29]    |
[01:09:29] 13 | extern crate rustc_driver;
[01:09:29]    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:09:29] 
[01:09:29] error: Could not compile `rls`.
