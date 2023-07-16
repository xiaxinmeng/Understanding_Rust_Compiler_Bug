
   Compiling aaaaaa v0.1.0 (/tmp/aaaaaa)
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

    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
