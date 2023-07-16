
/t/bar (master|…) $ cargo +nightly-2019-10-13 run
   Compiling bar v0.1.0 (/tmp/bar)
    Finished dev [unoptimized + debuginfo] target(s) in 0.38s
     Running `target/debug/bar`
[src/main.rs:3] cfg!(target_has_atomic = "cas") = true
[src/main.rs:4] cfg!(target_has_atomic = "ptr") = true

/t/bar (master|…) $ cargo +nightly-2019-10-14 run
   Compiling bar v0.1.0 (/tmp/bar)
    Finished dev [unoptimized + debuginfo] target(s) in 0.36s
     Running `target/debug/bar`
[src/main.rs:3] cfg!(target_has_atomic = "cas") = false
[src/main.rs:4] cfg!(target_has_atomic = "ptr") = true
