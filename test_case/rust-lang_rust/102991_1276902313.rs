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
........................................................................................ 264/651
........................................................................................ 352/651
........................................................................................ 440/651
........................................................................................ 528/651
..................................................F...................................F. 616/651
thread panicked while panicking. aborting.
error: test failed, to rerun pass `-p alloc --test collectionstests`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/collectionstests-77025343f1efe33a --quiet` (signal: 6, SIGABRT: process abort signal)
....FF.............F.....FBuild completed unsuccessfully in 0:01:09
