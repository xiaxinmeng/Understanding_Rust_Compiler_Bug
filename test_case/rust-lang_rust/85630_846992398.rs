plain
thread '<unnamed>' panicked at 'explicit panic', library/std/src/io/buffered/tests.rs:423:13
.................................................................................................... 300/861
thread '<unnamed>' panicked at 'explicit panic', library/std/src/io/stdio/tests.rs:36:9
.................................................................................................... 400/861
F..F....F......F....F.F.F....F.F.F.FF.............................F................................. 500/861
.................................................................................................... 600/861
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: SendError { .. }', library/std/src/sync/mpsc/sync_tests.rs:371:24
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', library/std/src/sync/mpsc/sync_tests.rs:250:19
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', library/std/src/sync/mpsc/sync_tests.rs:383:27
---
thread '<unnamed>' panicked at 'Box<Any>', library/std/src/thread/tests.rs:205:33
.............................................................
failures:

---- net::ip::tests::cmp stdout ----
thread 'net::ip::tests::cmp' panicked at 'called `Result::unwrap()` on an `Err` value: AddrParseError(())', library/std/src/net/ip/tests.rs:783:56
---- net::addr::tests::compare stdout ----
---- net::addr::tests::compare stdout ----
thread 'net::addr::tests::compare' panicked at 'called `Result::unwrap()` on an `Err` value: AddrParseError(())', library/std/src/net/addr/tests.rs:203:69
---- net::ip::tests::ip_properties stdout ----
---- net::ip::tests::ip_properties stdout ----
thread 'net::ip::tests::ip_properties' panicked at 'called `Result::unwrap()` on an `Err` value: AddrParseError(())', library/std/src/net/ip/tests.rs:299:5
---- net::ip::tests::ipv6_properties stdout ----
---- net::ip::tests::ipv6_properties stdout ----
thread 'net::ip::tests::ipv6_properties' panicked at 'called `Result::unwrap()` on an `Err` value: AddrParseError(())', library/std/src/net/ip/tests.rs:602:5
---- net::ip::tests::test_from_str_ipv6 stdout ----
---- net::ip::tests::test_from_str_ipv6 stdout ----
thread 'net::ip::tests::test_from_str_ipv6' panicked at 'assertion failed: `(left == right)`
  left: `Ok(::)`,
 right: `Err(AddrParseError(()))`', library/std/src/net/ip/tests.rs:27:5
---- net::ip::tests::test_from_str_ipv4_in_ipv6 stdout ----
---- net::ip::tests::test_from_str_ipv4_in_ipv6 stdout ----
thread 'net::ip::tests::test_from_str_ipv4_in_ipv6' panicked at 'assertion failed: `(left == right)`
  left: `Ok(::ffff:192.0.2.33)`,
 right: `Err(AddrParseError(()))`', library/std/src/net/ip/tests.rs:58:5
---- net::ip::tests::test_from_str_socket_addr stdout ----
---- net::ip::tests::test_from_str_socket_addr stdout ----
thread 'net::ip::tests::test_from_str_socket_addr' panicked at 'assertion failed: `(left == right)`
  left: `Ok([2a02:6b8:0:1::1]:53)`,
 right: `Err(AddrParseError(()))`', library/std/src/net/ip/tests.rs:83:5
---- net::parser::tests::ipv6_corner_cases stdout ----
---- net::parser::tests::ipv6_corner_cases stdout ----
thread 'net::parser::tests::ipv6_corner_cases' panicked at 'called `Result::unwrap()` on an `Err` value: AddrParseError(())', library/std/src/net/parser/tests.rs:113:42
---- net::parser::tests::parse_ip stdout ----
---- net::parser::tests::parse_ip stdout ----
thread 'net::parser::tests::parse_ip' panicked at 'called `Result::unwrap()` on an `Err` value: AddrParseError(())', library/std/src/net/parser/tests.rs:60:48
---- net::parser::tests::parse_socket stdout ----
---- net::parser::tests::parse_socket stdout ----
thread 'net::parser::tests::parse_socket' panicked at 'called `Result::unwrap()` on an `Err` value: AddrParseError(())', library/std/src/net/parser/tests.rs:102:52
---- net::parser::tests::parse_ipv6 stdout ----
---- net::parser::tests::parse_ipv6 stdout ----
thread 'net::parser::tests::parse_ipv6' panicked at 'called `Result::unwrap()` on an `Err` value: AddrParseError(())', library/std/src/net/parser/tests.rs:39:50
---- net::parser::tests::parse_socket_v6 stdout ----
---- net::parser::tests::parse_socket_v6 stdout ----
thread 'net::parser::tests::parse_socket_v6' panicked at 'assertion failed: `(left == right)`
  left: `Err(AddrParseError(()))`,
 right: `Ok([2001:db8::c0a8:1]:8080)`', library/std/src/net/parser/tests.rs:87:5
---- net::addr::tests::to_socket_addr_str stdout ----
---- net::addr::tests::to_socket_addr_str stdout ----
thread 'net::addr::tests::to_socket_addr_str' panicked at 'assertion failed: `(left == right)`
  left: `Ok([[2a02:6b8:0:1::1]:53])`,
 right: `Err("failed to lookup address information: Name or service not known")`', library/std/src/net/addr/tests.rs:33:5

failures:
    net::addr::tests::compare
    net::addr::tests::to_socket_addr_str
---
    net::parser::tests::parse_socket_v6

test result: FAILED. 848 passed; 13 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.29s

error: test failed, to rerun pass '-p std --lib'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "std" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:19:57
