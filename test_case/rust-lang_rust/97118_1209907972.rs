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

---- net::ip::tests::ip_properties stdout ----
thread 'net::ip::tests::ip_properties' panicked at 'called `Result::unwrap()` on an `Err` value: AddrParseError(Ip)', library/std/src/net/ip/tests.rs:315:5

failures:
    net::ip::tests::ip_properties


test result: FAILED. 931 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 10.32s

error: test failed, to rerun pass '-p std --lib'
Build completed unsuccessfully in 0:01:38
