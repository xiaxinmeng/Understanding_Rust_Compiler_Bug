
    Checking aaaaaa v0.1.0 (/tmp/aaaaaa)
warning: lifetime parameter `'a` never used
 --> src/main.rs:3:20
  |
3 | fn unused_lifetime<'a>() {}
  |                   -^^- help: elide the unused lifetime
  |
note: lint level defined here
 --> src/main.rs:1:9
  |
1 | #![warn(unused_lifetimes)]
  |         ^^^^^^^^^^^^^^^^

warning: this lifetime isn't used in the function definition
 --> src/main.rs:3:20
  |
3 | fn unused_lifetime<'a>() {}
  |                    ^^
  |
note: lint level defined here
 --> src/main.rs:2:9
  |
2 | #![warn(clippy::extra_unused_lifetimes)]
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#extra_unused_lifetimes

    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
