txt
Compiling playground v0.0.1 (/playground)
error[[E0599]](https://doc.rust-lang.org/stable/error-index.html#E0599): `Option<u32>` is not an iterator
   --> src/main.rs:7:16
    |
7   |           self.a.cloned()
    |                  ^^^^^^ `Option<u32>` is not an iterator
    |
    = note: the following trait bounds were not satisfied:
            `Option<u32>: Iterator`
            which is required by `&mut Option<u32>: Iterator`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `playground` due to previous error
