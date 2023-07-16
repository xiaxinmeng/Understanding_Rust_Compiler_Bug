console
error[E0015]: calls in constants are limited to constant functions, tuple structs and tuple variants
 --> src/main.rs:6:27
  |
6 | const TIMEOUT: Duration = 5 * Duration::SECOND;
  |                           ^^^^^^^^^^^^^^^^^^^^
