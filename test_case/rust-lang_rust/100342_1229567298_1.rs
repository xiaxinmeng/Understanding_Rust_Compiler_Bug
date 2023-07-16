rust
# cargo +source check
    Checking abcdef v0.1.0 (/tmp/scratchaBCBdAv6h)
warning: use of deprecated function `std::mem::uninitialized`: use `mem::MaybeUninit` instead
 --> src/main.rs:2:39
  |
2 |     let _x: char = unsafe { std::mem::uninitialized() };
  |                                       ^^^^^^^^^^^^^
  |
  = note: `#[warn(deprecated)]` on by default

warning: the type `char` does not permit being left uninitialized
 --> src/main.rs:2:29
  |
2 |     let _x: char = unsafe { std::mem::uninitialized() };
  |                             ^^^^^^^^^^^^^^^^^^^^^^^^^
  |                             |
  |                             this code causes undefined behavior when executed
  |                             help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
  |
  = note: `#[warn(mem_uninitialized)]` on by default
  = note: for more information, see FIXME: fill this in
  = note: characters must be a valid Unicode codepoint

warning: `abcdef` (bin "abcdef") generated 2 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
warning: the following packages contain code that will be rejected by a future version of Rust: abcdef v0.1.0 (/tmp/scratchaBCBdAv6h)
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 8`
