
op@VBOX ~/m/rust2> build/x86_64-unknown-linux-gnu/stage1/bin/rustc src/test/ui/parser/raw/raw-literal-too-many.rs
error: too many `#` when terminating raw string
 --> src/test/ui/parser/raw/raw-literal-too-many.rs:2:23
  |
2 |     let foo = r##"bar"####; //~ERROR too many `#` when terminating raw string
  |                --     ^^^^ ...but is closed with 4.
  |                |
  |                The raw string has 2 leading `#`...
  = help: remove the unneeded `#`

error: aborting due to previous error

op@VBOX ~/m/rust2> build/x86_64-unknown-linux-gnu/stage1/bin/rustc src/test/ui/parser/raw/raw-literal-too-many-long.rs 
error: too many `#` when terminating raw string
  --> src/test/ui/parser/raw/raw-literal-too-many-long.rs:20:6
   |
2  |     let a = r##"This
   |              -- The raw string has 2 leading `#`...
...
20 |     "###; //~ERROR too many `#` when terminating raw string"
   |      ^^^ ...but is closed with 3.
   = help: remove the unneeded `#`

error: aborting due to previous error
