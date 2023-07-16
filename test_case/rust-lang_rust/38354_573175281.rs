
$ echo -e "'t.'\n\n" > file.rs
$ rustc file.rs
error: unterminated character literal
 --> file.rs:1:4
  |
1 | 't.'
  |    ^

error: aborting due to previous error
