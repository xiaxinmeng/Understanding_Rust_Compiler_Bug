plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
Checking which error codes lack tests...
Found 434 error codes
Found 0 error codes with no tests
Done!
tidy error: `/checkout/library/alloc/src/alloc/struct_alloc/test.rs:117` contains `#[test]`; unit tests and benchmarks must be placed into separate files or directories named `tests.rs`, `benches.rs`, `tests` or `benches`
tidy error: /checkout/library/alloc/src/alloc/struct_alloc.rs:18: unexplained "