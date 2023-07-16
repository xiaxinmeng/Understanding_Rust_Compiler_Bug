
$ cargo doc --no-deps -v
 Documenting b v0.1.0 (/home/joshua/src/test-rustdoc/b)
     Running `rustdoc --crate-type lib --crate-name b src/lib.rs -o /home/joshua/src/test-rustdoc/b/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/home/joshua/src/test-rustdoc/b/debug/deps --extern a=/home/joshua/src/test-rustdoc/b/debug/deps/liba-d962d8f2c027d5fc.rmeta`
$ cargo doc -v
 Documenting b v0.1.0 (/home/joshua/src/test-rustdoc/b)
     Running `rustdoc --crate-type lib --crate-name b src/lib.rs -o /home/joshua/src/test-rustdoc/b/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/home/joshua/src/test-rustdoc/b/debug/deps --extern a=/home/joshua/src/test-rustdoc/b/debug/deps/liba-d962d8f2c027d5fc.rmeta`
