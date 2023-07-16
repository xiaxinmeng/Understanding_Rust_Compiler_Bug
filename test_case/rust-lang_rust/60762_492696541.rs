
error: too many `#` when terminating raw string
 --> src/test/ui/parser/raw/raw-literal-too-many.rs:2:15
  |
2 |     let foo = r##"bar"####;
  |                --     ^^^^
  |                |      |
  |                |      ...but it's closed with 4
  |                the raw string has 2 leading `#`...
