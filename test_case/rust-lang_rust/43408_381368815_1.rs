
   Compiling playground v0.0.1 (file:///playground)
warning: constant evaluation error: the type `Self` has an unknown layout
 --> src/main.rs:2:25
  |
2 |     const SIZE: usize = ::std::mem::size_of::<Self>();
  |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(const_err)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.52 secs
     Running `target/debug/playground`
