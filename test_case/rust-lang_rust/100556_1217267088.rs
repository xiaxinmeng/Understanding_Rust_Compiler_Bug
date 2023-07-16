plain
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 144 tests
iii..........iiiiiiiiiii................................................................ 88/144
...................................2022-08-16T22:25:03.870808Z ERROR compiletest::runtest: fatal error, panic: "missing 'assembly-output' header"
Some tests failed in compiletest suite=assembly mode=assembly host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
F.iii................


---- [assembly] src/test/assembly/x86_64-floating-point-clamp.rs stdout ----

error: missing 'assembly-output' header
thread '[assembly] src/test/assembly/x86_64-floating-point-clamp.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2225:9


failures:
failures:
    [assembly] src/test/assembly/x86_64-floating-point-clamp.rs
test result: FAILED. 126 passed; 1 failed; 17 ignored; 0 measured; 0 filtered out; finished in 0.55s

Build completed unsuccessfully in 0:11:38
