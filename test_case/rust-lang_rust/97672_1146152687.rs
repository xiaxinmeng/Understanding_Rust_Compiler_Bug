plain
thread '<unnamed>' panicked at 'static string', library/std/src/thread/tests.rs:190:9
................................................
failures:

---- net::udp::tests::set_reuseaddr_v4_not_windows stdout ----
thread 'net::udp::tests::set_reuseaddr_v4_not_windows' panicked at 'received error for `unbound_socket4.bind(&addr)`: Address already in use (os error 98)', library/std/src/net/udp/tests.rs:392:18

---- net::udp::tests::set_reuseaddr_v6_not_windows stdout ----
thread 'net::udp::tests::set_reuseaddr_v6_not_windows' panicked at 'received error for `unbound_socket4.bind(&addr)`: Address already in use (os error 98)', library/std/src/net/udp/tests.rs:445:18

failures:
failures:
    net::udp::tests::set_reuseaddr_v4_not_windows
    net::udp::tests::set_reuseaddr_v6_not_windows
test result: FAILED. 925 passed; 2 failed; 1 ignored; 0 measured; 0 filtered out; finished in 10.37s


error: test failed, to rerun pass '-p std --lib'
