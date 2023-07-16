
$  cargo bench
   Compiling either v1.5.3
   Compiling foo v0.1.0 (/tmp/foo)
    Finished bench [optimized] target(s) in 0.55s
     Running target/release/deps/foo-f86bbdc9426b849c

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/release/deps/test-33fbe6ef386a2878

running 2 tests
test tests::test_collect  ... bench:     129,910 ns/iter (+/- 1,603)
test tests::test_for_each ... bench:     157,252 ns/iter (+/- 2,265)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured; 0 filtered out

$ rustc -vV                                           
rustc 1.40.0-nightly (4a8c5b20c 2019-10-23)
binary: rustc
commit-hash: 4a8c5b20c7772bc5342b83d4b0696ea216ef75a7
commit-date: 2019-10-23
host: x86_64-unknown-linux-gnu
release: 1.40.0-nightly
LLVM version: 9.0
