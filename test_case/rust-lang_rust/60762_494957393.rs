
error: too many `#` when terminating raw string
  --> src/test/ui/parser/raw/raw-literal-too-many-long.rs:2:13
   |
2  |       let a = r##"This
   |                -- this raw string has 2 leading `#`
...
20 |      "###;
   |       ^^^ this raw string is closed with 3 trailing `#`, but should be 2
   = help: remove the unneeded `#`