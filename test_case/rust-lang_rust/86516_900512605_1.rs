
$ cargo +nightly rustc -- -Zunstable-options --force-warn dead_code
   Compiling deleteme v0.1.0 (/home/rylevick/deleteme)
warning: function is never used: `foo`
 --> src/main.rs:6:4
  |
6 | fn foo() {}
  |    ^^^
  |
  = note: requested on the command line with `--force-warn dead-code`

warning: `deleteme` (bin "deleteme") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s

$  cargo +nightly rustc
   Compiling deleteme v0.1.0 (/home/rylevick/deleteme)
error: function is never used: `foo`
 --> src/main.rs:6:4
  |
6 | fn foo() {}
  |    ^^^
  |
note: the lint level is defined here
 --> src/main.rs:1:9
  |
1 | #![deny(dead_code)]
  |         ^^^^^^^^^

error: could not compile `deleteme` due to previous error
