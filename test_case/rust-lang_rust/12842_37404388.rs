
failures:

---- io::net::tcp::test::multiple_connect_serial_ip6::green stdout ----
    task 'io::net::tcp::test::multiple_connect_serial_ip6::green' failed at 'called `Result::unwrap()` on an `Err` value', src\libstd\result.rs:187

---- io::net::tcp::test::multiple_connect_serial_ip6::native stdout ----
    task 'io::net::tcp::test::multiple_connect_serial_ip6::native' failed at 'receiving on a closed channel', src\libstd\comm\mod.rs:507

---- io::net::tcp::test::read_eof_twice_ip6::green stdout ----
    task 'io::net::tcp::test::read_eof_twice_ip6::green' failed at 'unknown kind: OtherIoError', src\libstd\io\net\tcp.rs:294

---- io::net::tcp::test::read_eof_twice_ip6::native stdout ----
    task 'io::net::tcp::test::read_eof_twice_ip6::native' failed at 'receiving on a closed channel', src\libstd\comm\mod.rs:507

---- io::net::tcp::test::smoke_test_ip6::green stdout ----
    task 'io::net::tcp::test::smoke_test_ip6::green' failed at 'called `Result::unwrap()` on an `Err` value', src\libstd\result.rs:187

---- io::net::tcp::test::smoke_test_ip6::native stdout ----
    task 'io::net::tcp::test::smoke_test_ip6::native' failed at 'receiving on a closed channel', src\libstd\comm\mod.rs:507

---- io::net::tcp::test::socket_and_peer_name_ip6::green stdout ----
    task 'io::net::tcp::test::socket_and_peer_name_ip6::green' failed at 'called `Result::unwrap()` on an `Err` value', src\libstd\result.rs:187

---- io::net::tcp::test::socket_and_peer_name_ip6::native stdout ----
    task 'io::net::tcp::test::socket_and_peer_name_ip6::native' failed at 'receiving on a closed channel', src\libstd\comm\mod.rs:507

---- io::net::tcp::test::tcp_clone_two_read::green stdout ----
    task 'io::net::tcp::test::tcp_clone_two_read::green' failed at 'called `Result::unwrap()` on an `Err` value', src\libstd\result.rs:187

---- io::net::tcp::test::tcp_clone_two_read::native stdout ----
    task 'io::net::tcp::test::tcp_clone_two_read::native' failed at 'receiving on a closed channel', src\libstd\comm\mod.rs:507

---- io::net::tcp::test::write_close_ip6::green stdout ----
    task 'io::net::tcp::test::write_close_ip6::green' failed at 'unknown error: io::IoError{kind: OtherIoError, desc: "address family not supported", detail: None}', src\libstd\io\net\tcp.rs:348

---- io::net::tcp::test::write_close_ip6::native stdout ----
    task 'io::net::tcp::test::write_close_ip6::native' failed at 'receiving on a closed channel', src\libstd\comm\mod.rs:507

---- io::net::udp::test::socket_name_ip6::green stdout ----
    task 'io::net::udp::test::socket_name_ip6::green' failed at 'assertion failed: server.is_ok()', src\libstd\io\net\udp.rs:253

---- io::net::udp::test::socket_name_ip6::native stdout ----
    task 'io::net::udp::test::socket_name_ip6::native' failed at 'receiving on a closed channel', src\libstd\comm\mod.rs:507

---- io::net::udp::test::socket_smoke_test_ip6::green stdout ----
    task 'io::net::udp::test::socket_smoke_test_ip6::green' failed at 'explicit failure', src\libstd\io\net\udp.rs:142

---- io::net::udp::test::socket_smoke_test_ip6::native stdout ----
    task 'io::net::udp::test::socket_smoke_test_ip6::native' failed at 'receiving on a closed channel', src\libstd\comm\mod.rs:507

---- io::net::udp::test::stream_smoke_test_ip6::green stdout ----
    task 'io::net::udp::test::stream_smoke_test_ip6::green' failed at 'explicit failure', src\libstd\io\net\udp.rs:212

---- io::net::udp::test::stream_smoke_test_ip6::native stdout ----
    task 'io::net::udp::test::stream_smoke_test_ip6::native' failed at 'receiving on a closed channel', src\libstd\comm\mod.rs:507

---- io::process::tests::detach_works::green stdout ----
    task 'io::process::tests::detach_works::green' failed at 'called `Result::unwrap()` on an `Err` value', src\libstd\result.rs:187

---- io::process::tests::exit_reported_right::green stdout ----
    task 'io::process::tests::exit_reported_right::green' failed at 'assertion failed: p.is_ok()', src\libstd\io\process.rs:454

---- io::process::tests::detach_works::native stdout ----
    task 'io::process::tests::detach_works::native' failed at 'receiving on a closed channel', src\libstd\comm\mod.rs:507

---- io::process::tests::smoke::green stdout ----
    task 'io::process::tests::smoke::green' failed at 'assertion failed: p.is_ok()', src\libstd\io\process.rs:430

---- io::process::tests::exit_reported_right::native stdout ----
    task 'io::process::tests::exit_reported_right::native' failed at 'receiving on a closed channel', src\libstd\comm\mod.rs:507

---- io::process::tests::smoke::native stdout ----
    task 'io::process::tests::smoke::native' failed at 'receiving on a closed channel', src\libstd\comm\mod.rs:507

---- io::process::tests::stdout_works::green stdout ----
    task 'io::process::tests::stdout_works::green' failed at 'assertion failed: p.is_ok()', src\libstd\io\process.rs:487

---- io::process::tests::stdout_works::native stdout ----
    task 'io::process::tests::stdout_works::native' failed at 'receiving on a closed channel', src\libstd\comm\mod.rs:507

---- io::process::tests::test_finish_once::green stdout ----
    task 'io::process::tests::test_finish_once::green' failed at 'called `Result::unwrap()` on an `Err` value', src\libstd\result.rs:187

---- io::process::tests::test_finish_once::native stdout ----
    task 'io::process::tests::test_finish_once::native' failed at 'receiving on a closed channel', src\libstd\comm\mod.rs:507

---- io::process::tests::test_finish_twice::green stdout ----
    task 'io::process::tests::test_finish_twice::green' failed at 'called `Result::unwrap()` on an `Err` value', src\libstd\result.rs:187

---- io::process::tests::test_finish_twice::native stdout ----
    task 'io::process::tests::test_finish_twice::native' failed at 'receiving on a closed channel', src\libstd\comm\mod.rs:507

---- io::process::tests::test_process_output_error::green stdout ----
    task 'io::process::tests::test_process_output_error::green' failed at 'called `Result::unwrap()` on an `Err` value', src\libstd\result.rs:187

---- io::process::tests::test_process_output_error::native stdout ----
    task 'io::process::tests::test_process_output_error::native' failed at 'receiving on a closed channel', src\libstd\comm\mod.rs:507

---- io::process::tests::test_kill::native stdout ----
    task 'io::process::tests::test_kill::native' failed at 'receiving on a closed channel', src\libstd\comm\mod.rs:507

---- io::process::tests::test_process_output_output::green stdout ----
    task 'io::process::tests::test_process_output_output::green' failed at 'called `Result::unwrap()` on an `Err` value', src\libstd\result.rs:187

---- io::process::tests::test_process_output_output::native stdout ----
    task 'io::process::tests::test_process_output_output::native' failed at 'receiving on a closed channel', src\libstd\comm\mod.rs:507

---- io::process::tests::test_process_status::green stdout ----
    task 'io::process::tests::test_process_status::green' failed at 'called `Result::unwrap()` on an `Err` value', src\libstd\result.rs:187

---- io::process::tests::test_wait_with_output_once::green stdout ----
    task 'io::process::tests::test_wait_with_output_once::green' failed at 'called `Result::unwrap()` on an `Err` value', src\libstd\result.rs:187

---- io::process::tests::test_wait_with_output_once::native stdout ----
    task 'io::process::tests::test_wait_with_output_once::native' failed at 'receiving on a closed channel', src\libstd\comm\mod.rs:507

---- io::process::tests::test_wait_with_output_twice::green stdout ----
    task 'io::process::tests::test_wait_with_output_twice::green' failed at 'called `Result::unwrap()` on an `Err` value', src\libstd\result.rs:187

---- io::process::tests::test_process_status::native stdout ----
    task 'io::process::tests::test_process_status::native' failed at 'receiving on a closed channel', src\libstd\comm\mod.rs:507

---- io::process::tests::test_wait_with_output_twice::native stdout ----
    task 'io::process::tests::test_wait_with_output_twice::native' failed at 'receiving on a closed channel', src\libstd\comm\mod.rs:507

test result: FAILED. 1276 passed; 41 failed; 134 ignored; 0 measured
