
[misdreavus@tonberry asdf]$ rustc --crate-type lib --crate-name asdf src/main.rs
[misdreavus@tonberry asdf]$ rustdoc --test --crate-name asdf src/main.rs --extern asdf=libasdf.rlib

running 1 test
test src/main.rs - SomeStruct (line 3) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
