
Compiling playground v0.0.1 (/playground)
error: invalid `struct` delimiters or `fn` call arguments
 [--> src/main.rs:7:5
](https://play.rust-lang.org/#)  |
7 |     tokio::time::sleep(Duration:from_millis(200))
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
help: if `tokio::time::sleep` is a struct, use braces as delimiters
  |
7 |     tokio::time::sleep { Duration:from_millis(200) }
  |                        ~                           ~
help: if `tokio::time::sleep` is a function, use the arguments directly
  |
7 -     tokio::time::sleep(Duration:from_millis(200))
7 +     tokio::time::sleep(from_millis(200))
  | 

error: could not compile `playground` due to previous error
