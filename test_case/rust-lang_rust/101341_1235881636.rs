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

running 373 tests
........................................................................................ 88/373
........................................................................................ 176/373
memory allocation of 503 bytes failed
error: test failed, to rerun pass '-p alloc --lib'
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/alloc-9c84c192afab5a22 --quiet` (signal: 6, SIGABRT: process abort signal)
.......................................................................................Build completed unsuccessfully in 0:01:27
