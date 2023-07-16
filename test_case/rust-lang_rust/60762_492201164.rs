
error: unterminated raw string
  --> $DIR/raw-string-long.rs:2:13
   |
LL |     let a = r##"This
   |             ^
   |             started here with 2 `#`
...
   |
LL |        ends"#;
   |             ^ ended here with only 1 `#`
   |             help: close the raw string with `"##`

error: aborting due to previous error
