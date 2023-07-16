plain
test fs::tests::read_large_dir ... ok

failures:

---- sys::unix::kernel_copy::tests::bench_file_to_socket_copy stdout ----
thread 'sys::unix::kernel_copy::tests::bench_file_to_socket_copy' panicked at 'called `Result::unwrap()` on an `Err` value: Custom { kind: Uncategorized, error: "failed to lookup address information: No address associated with hostname" }', library/std/src/sys/unix/kernel_copy/tests.rs:130:69

failures:
    sys::unix::kernel_copy::tests::bench_file_to_socket_copy


test result: FAILED. 931 passed; 1 failed; 4 ignored; 0 measured; 0 filtered out; finished in 952.69s

error: test failed, to rerun pass '-p std --lib'
