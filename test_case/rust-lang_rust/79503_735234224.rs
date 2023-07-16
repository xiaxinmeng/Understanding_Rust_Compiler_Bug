
→ rustup show active-toolchain
stable-x86_64-apple-darwin (default)
→ cargo test -p clap_derive --test nested --features "wrap_help yaml regex"
   Compiling ....
    Finished test [unoptimized + debuginfo] target(s) in 55.55s
     Running target/debug/deps/nested-55411ec307e7b9f1

running 1 test
test use_option ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
