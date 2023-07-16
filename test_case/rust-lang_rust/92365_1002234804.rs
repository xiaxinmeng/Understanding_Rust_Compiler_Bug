plain
    Checking hashbrown v0.11.0
    Checking object v0.26.2
    Checking miniz_oxide v0.4.0
    Checking addr2line v0.16.0
error: use of function `env::remove_var` that will be deprecated in a future Rust version: method is unsound
   |
46 |                 env::remove_var(k);
   |                      ^^^^^^^^^^
   |
   |
   = note: `-D deprecated-in-future` implied by `-D warnings`

error: use of function `env::set_var` that will be deprecated in a future Rust version: method is unsound
   |
51 |                 env::set_var(key, val);
   |                      ^^^^^^^


error: use of function `env::remove_var` that will be deprecated in a future Rust version: method is unsound
   |
53 |                 env::remove_var(key);
   |                      ^^^^^^^^^^

