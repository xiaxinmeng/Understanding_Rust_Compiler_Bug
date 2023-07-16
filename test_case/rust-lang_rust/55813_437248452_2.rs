
$ rustdoc --test foo.rs --test-args --nocapture

running 1 test
error: expected type, found `123`
 --> foo.rs:2:8
  |
3 | Input: 123
  |        ^^^ expecting a type here because of type ascription

test foo.rs - foo (line 1) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
