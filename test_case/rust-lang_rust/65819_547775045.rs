rust
error: this .into_iter() call is equivalent to .iter() and will not move the array
   --> src\decoder.rs:235:47
    |
235 |                     for (i, &table) in tables.into_iter().enumerate() {
    |                                               ^^^^^^^^^ help: call directly: `iter`
    |
    = note: `#[deny(clippy::into_iter_on_array)]` on by default
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#into_iter_on_array
