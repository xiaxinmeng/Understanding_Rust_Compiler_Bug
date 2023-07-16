plain
test sync::mpsc::tests::stress_recv_timeout_two_threads ... ok

failures:

---- net::tcp::tests::clone_accept_smoke stdout ----
thread 'net::tcp::tests::clone_accept_smoke' panicked at 'received error for `a.try_clone()`: The operation completed successfully. (os error 0)', library\std\src\net\tcp\tests.rs:582:18
---- net::tcp::tests::clone_accept_concurrent stdout ----
---- net::tcp::tests::clone_accept_concurrent stdout ----
thread 'net::tcp::tests::clone_accept_concurrent' panicked at 'received error for `a.try_clone()`: The operation completed successfully. (os error 0)', library\std\src\net\tcp\tests.rs:600:18
---- net::tcp::tests::close_readwrite_smoke stdout ----
---- net::tcp::tests::close_readwrite_smoke stdout ----
thread 'net::tcp::tests::close_readwrite_smoke' panicked at 'received error for `s.try_clone()`: The operation completed successfully. (os error 0)', library\std\src\net\tcp\tests.rs:484:22
---- net::tcp::tests::clone_while_reading stdout ----
---- net::tcp::tests::clone_while_reading stdout ----
thread 'net::tcp::tests::clone_while_reading' panicked at 'received error for `tcp.try_clone()`: The operation completed successfully. (os error 0)', library\std\src\net\tcp\tests.rs:556:20
---- net::tcp::tests::tcp_clone_smoke stdout ----
---- net::tcp::tests::tcp_clone_smoke stdout ----
thread 'net::tcp::tests::tcp_clone_smoke' panicked at 'received error for `s1.try_clone()`: The operation completed successfully. (os error 0)', library\std\src\net\tcp\tests.rs:370:18
---- net::tcp::tests::tcp_clone_two_read stdout ----
---- net::tcp::tests::tcp_clone_two_read stdout ----
thread 'net::tcp::tests::tcp_clone_two_read' panicked at 'received error for `s1.try_clone()`: The operation completed successfully. (os error 0)', library\std\src\net\tcp\tests.rs:403:18
---- net::tcp::tests::tcp_clone_two_write stdout ----
---- net::tcp::tests::tcp_clone_two_write stdout ----
thread 'net::tcp::tests::tcp_clone_two_write' panicked at 'received error for `s1.try_clone()`: The operation completed successfully. (os error 0)', library\std\src\net\tcp\tests.rs:434:18
---- net::udp::tests::udp_clone_smoke stdout ----
---- net::udp::tests::udp_clone_smoke stdout ----
thread 'net::udp::tests::udp_clone_smoke' panicked at 'received error for `sock1.try_clone()`: The operation completed successfully. (os error 0)', library\std\src\net\udp\tests.rs:85:21
---- net::udp::tests::udp_clone_two_read stdout ----
---- net::udp::tests::udp_clone_two_read stdout ----
thread 'net::udp::tests::udp_clone_two_read' panicked at 'received error for `sock1.try_clone()`: The operation completed successfully. (os error 0)', library\std\src\net\udp\tests.rs:116:21
---- net::udp::tests::udp_clone_two_write stdout ----
---- net::udp::tests::udp_clone_two_write stdout ----
thread 'net::udp::tests::udp_clone_two_write' panicked at 'received error for `sock1.try_clone()`: The operation completed successfully. (os error 0)', library\std\src\net\udp\tests.rs:149:21

failures:
    net::tcp::tests::clone_accept_concurrent
    net::tcp::tests::clone_accept_smoke
---
    net::udp::tests::udp_clone_two_write

test result: FAILED. 873 passed; 10 failed; 3 ignored; 0 measured; 0 filtered out; finished in 14.09s

error: test failed, to rerun pass '-p std --lib'


command did not execute successfully: "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "test" "--target" "x86_64-pc-windows-msvc" "-Zbinary-dep-depinfo" "-j" "8" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "D:\\a\\rust\\rust\\library/test/Cargo.toml" "-p" "std" "--"


Build completed unsuccessfully in 0:55:14
Build completed unsuccessfully in 0:55:14
make: *** [Makefile:72: ci-subset-1] Error 1
