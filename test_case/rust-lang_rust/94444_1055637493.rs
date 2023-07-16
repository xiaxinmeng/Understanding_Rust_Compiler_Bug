`
~/.rustup/toolchains/master/bin/rustc 94444.rs
error[E0061]: this enum variant takes 1 argument but 2 arguments were supplied
  --> 94444.rs:10:13
   |
10 |             Some(t.into_iter().next().unwrap(), 1)
   |             ^^^^ -----------------------------  - supplied 2 arguments
   |
help: use parentheses to construct a tuple
   |
10 |             Some((t.into_iter().next().unwrap(), 1))
   |                  +                                +

error: aborting due to previous error

For more information about this error, try `rustc --explain E0061`.
