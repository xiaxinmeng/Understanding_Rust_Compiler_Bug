
$ cargo +nightly build
   Compiling wut v0.1.0 (/home/alex/code/wut)
error: custom attribute panicked
 --> src/main.rs:2:5
  |
2 |     #[the_macro::the_macro]
  |     ^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: message: assertion failed: `(left == right)`
            left: `"type"`,
           right: `"pub"`

error: aborting due to previous error

error: could not compile `wut`.

To learn more, run the command again with --verbose.
