
[misdreavus@tonberry asdf]$ cargo test --doc --verbose
   Compiling asdf v0.1.0 (file:///home/misdreavus/git/asdf)
     Running `rustc --crate-name asdf src/lib.rs --crate-type lib --emit=dep-info,link -C debuginfo=2 -C metadata=31e1a62d36de9a99 -C extra-filename=-31e1a62d36de9a99 --out-dir /home/misdreavus/git/asdf/target/debug/deps -C incremental=/home/misdreavus/git/asdf/target/debug/incremental -L dependency=/home/misdreavus/git/asdf/target/debug/deps`
    Finished dev [unoptimized + debuginfo] target(s) in 0.79 secs
   Doc-tests asdf
     Running `rustdoc --test /home/misdreavus/git/asdf/src/lib.rs --crate-name asdf -L dependency=/home/misdreavus/git/asdf/target/debug/deps -L dependency=/home/misdreavus/git/asdf/target/debug/deps --extern asdf=/home/misdreavus/git/asdf/target/debug/deps/libasdf-31e1a62d36de9a99.rlib`

running 1 test
test src/lib.rs - SomeStruct (line 3) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
