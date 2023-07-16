
anon@anon:~/code/rust/example$ cargo +stage1 test
   Compiling example v0.1.0 (/home/anon/code/rust/example)
    Finished test [unoptimized + debuginfo] target(s) in 0.26s
     Running target/debug/deps/example-745b64d3885c3565

running 3 tests
test tests::foo ... ok
test tests::bar ... ok
test tests::baz ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.20s

   Doc-tests example

running 3 tests
test src/lib.rs - foo (line 3) ... ok
test src/lib.rs - bar (line 11) ... ok
test src/lib.rs - baz (line 19) ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.32s
