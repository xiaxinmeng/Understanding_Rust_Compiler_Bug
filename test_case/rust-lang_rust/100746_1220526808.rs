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
........................................................................................ 264/637
error: test failed, to rerun pass '-p alloc --test collectionstests'

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/collectionstests-69db20fd75bc5369 --quiet` (signal: 11, SIGSEGV: invalid memory reference)
.............................................................................Build completed unsuccessfully in 0:01:35
