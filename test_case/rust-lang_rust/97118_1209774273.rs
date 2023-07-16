plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
thread '<unnamed>' panicked at 'static string', library/std/src/thread/tests.rs:190:9
.....................................................
failures:

---- net::addr::tests::compare stdout ----
thread 'net::addr::tests::compare' panicked at 'called `Result::unwrap()` on an `Err` value: AddrParseError(SocketV4)', library/std/src/net/addr/tests.rs:200:61
---- net::parser::tests::parse_socket stdout ----
---- net::parser::tests::parse_socket stdout ----
thread 'net::parser::tests::parse_socket' panicked at 'called `Result::unwrap()` on an `Err` value: AddrParseError(Socket)', library/std/src/net/parser/tests.rs:99:52
---- net::parser::tests::parse_socket_v4 stdout ----
---- net::parser::tests::parse_socket_v4 stdout ----
thread 'net::parser::tests::parse_socket_v4' panicked at 'called `Result::unwrap()` on an `Err` value: AddrParseError(SocketV4)', library/std/src/net/parser/tests.rs:75:54

failures:
    net::addr::tests::compare
    net::parser::tests::parse_socket
    net::parser::tests::parse_socket
    net::parser::tests::parse_socket_v4

test result: FAILED. 929 passed; 3 failed; 1 ignored; 0 measured; 0 filtered out; finished in 10.32s

error: test failed, to rerun pass '-p std --lib'
Build completed unsuccessfully in 0:01:37
