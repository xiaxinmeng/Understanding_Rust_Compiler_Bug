
op@VBOX ~/m/rust2> build/x86_64-unknown-linux-gnu/stage1/bin/rustc src/test/ui/parser/raw/raw-literal-too-many-long.rs
error: too many `#` when terminating raw string
  --> src/test/ui/parser/raw/raw-literal-too-many-long.rs:2:13
   |
2  |       let a = r##"This
   |               ^-- The raw string has 2 leading `#`
   |  _____________|
   | |
3  | |     is
4  | |     a
5  | |     very
...  |
19 | |     lines
20 | |     "###;
   | |        ^
   | |        |
   | |________help: remove the unneeded `#`
   |          The raw string needs 2 trailing `#`, but found 3

op@VBOX ~/m/rust2> build/x86_64-unknown-linux-gnu/stage1/bin/rustc src/test/ui/parser/raw/raw-literal-too-many.rs
error: too many `#` when terminating raw string
 --> src/test/ui/parser/raw/raw-literal-too-many.rs:2:15
  |
2 |     let foo = r##"bar"####;
  |               ^--^^^^^^^--
  |               ||        |
  |               ||        help: remove the unneeded `#`
  |               |The raw string has 2 leading `#`
  |               The raw string needs 2 trailing `#`, but found 4
