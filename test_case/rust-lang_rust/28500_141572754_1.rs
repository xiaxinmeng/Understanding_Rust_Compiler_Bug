
failures:

---- sys::process::tests::test_process_mask stdout ----
    thread 'sys::process::tests::test_process_mask' panicked at 'called `Result::unwrap()` on an `Err` value: Error { repr: Os { code: 13, message: "Permission denied" } }', src/libcore/result.rs:736



failures:
    net::udp::tests::bind_error
    process::tests::signal_reported_right
    sys::process::tests::test_process_mask
