
$ cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/testprinttest-777d764775ed5046)

running 3 tests
Error: "test3 return"
Error: "test1 return"
test test3 ... FAILED
Error: "test2 return"
test test1 ... FAILED
test test2 ... FAILED

failures:

---- test3 stdout ----
test3 stdout
test3 stderr

---- test1 stdout ----
test1 stdout
test1 stderr

---- test2 stdout ----
test2 stdout
test2 stderr


failures:
    test1
    test2
    test3

test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--lib`
