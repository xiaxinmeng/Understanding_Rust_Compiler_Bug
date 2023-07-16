
$ cargo +nightly-2018-04-30 test --manifest-path issue50867/Cargo.toml  
    Finished dev [unoptimized + debuginfo] target(s) in 0.35 secs
     Running issue50867/target/debug/deps/issue50867-89d6e487ff6882a0

running 1 test
test test_transform_compress_none ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo +nightly-2018-05-03 test --manifest-path issue50867/Cargo.toml 
    Finished dev [unoptimized + debuginfo] target(s) in 0.38 secs
     Running issue50867/target/debug/deps/issue50867-eff7d2b2573ec7f4

running 1 test
error: process didn't exit successfully: `/Users/servo/simon/simon/issue50867/target/debug/deps/issue50867-eff7d2b2573ec7f4` (signal: 11, SIGSEGV: invalid memory reference)

$ cargo +nightly-2018-05-30 test --manifest-path issue50867/Cargo.toml              
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running issue50867/target/debug/deps/issue50867-efe70058af46c68e

running 1 test
test test_transform_compress_none ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
