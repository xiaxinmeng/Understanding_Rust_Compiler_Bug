
error: too many `#` when terminating raw string
 --> src/test/ui/parser/raw/parse_petro.rs:5:4
  |
5 | m!(r#"abc"##);
  |    ^-^^^^^--
  |     |     |
  |     |     ...but is closed with 2.
  |     this raw string has 1 leading `#`...
  = help: remove the unneeded `#`

error: this file contains an un-closed delimiter
 --> src/test/ui/parser/raw/parse_petro.rs:5:16
  |
1 | macro_rules!( m {
  |             - un-closed delimiter
...
5 | m!(r#"abc"##);
  |                ^

error: expected identifier, found `(`
 --> src/test/ui/parser/raw/parse_petro.rs:1:13
  |
1 | macro_rules!( m {
  |             ^ expected identifier

error: aborting due to 3 previous errors
