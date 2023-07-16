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
........................................................................................ 352/964
thread '<unnamed>' panicked at 'explicit panic', library/std/src/io/buffered/tests.rs:497:13
thread '<unnamed>' panicked at 'explicit panic', library/std/src/io/stdio/tests.rs:37:9
........................................................................................ 440/964
error: test failed, to rerun pass `-p std --lib`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/std-8de749a7f8487b59 --quiet` (signal: 11, SIGSEGV: invalid memory reference)
..............................................................................Build completed unsuccessfully in 0:01:22
