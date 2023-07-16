
> export RUSTFLAGS="-Ctarget-feature=+crt-static"
> cargo run
   Compiling test-weak-linkage v0.1.0 (/tmp/test-weak-linkage)
    Finished dev [unoptimized + debuginfo] target(s) in 0.85s
     Running `target/debug/test-weak-linkage`
true
> export RUSTFLAGS="-Ctarget-feature=-crt-static"
> cargo run
   Compiling test-weak-linkage v0.1.0 (/tmp/test-weak-linkage)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/test-weak-linkage`
false
