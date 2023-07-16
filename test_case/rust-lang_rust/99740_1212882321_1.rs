
> export RUSTFLAGS="-Ctarget-feature=-crt-static"
> cargo run
   Compiling test-weak-linkage v0.1.0 (/tmp/test-weak-linkage)
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `/var/tmp/rust/debug/test-weak-linkage`
false
> export RUSTFLAGS="-Ctarget-feature=+crt-static"
> cargo run
   Compiling test-weak-linkage v0.1.0 (/tmp/test-weak-linkage)
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `/var/tmp/rust/debug/test-weak-linkage`
true
