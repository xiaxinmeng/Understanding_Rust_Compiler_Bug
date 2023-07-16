plain
configure: llvm.link-shared     := True
configure: rust.thin-lto-import-instr-limit := 10
configure: rust.codegen-units-std := 1
configure: rust.verify-llvm-ir  := True
configure: llvm.download-ci-llvm := if-available
configure: build.submodules     := False
configure: build.locked-deps    := True
configure: build.cargo-native-static := True
configure: dist.compression-formats := ['xz']
---
   Compiling ignore v0.4.18
   Compiling xz2 v0.1.6
   Compiling toml v0.5.9
    Finished dev [unoptimized] target(s) in 48.91s
thread 'main' panicked at 'setting llvm.link_shared is incompatible with download-ci-llvm.', config.rs:1010:17
Build completed unsuccessfully in 0:01:08
make: *** [Makefile:58: prepare] Error 1
Command failed. Attempt 2/5:
Building rustbuild
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.05s
thread 'main' panicked at 'setting llvm.link_shared is incompatible with download-ci-llvm.', config.rs:1010:17
Build completed unsuccessfully in 0:00:00
make: *** [Makefile:58: prepare] Error 1
Command failed. Attempt 3/5:
Building rustbuild
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.05s
thread 'main' panicked at 'setting llvm.link_shared is incompatible with download-ci-llvm.', config.rs:1010:17
Build completed unsuccessfully in 0:00:00
make: *** [Makefile:58: prepare] Error 1
Command failed. Attempt 4/5:
Building rustbuild
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.06s
thread 'main' panicked at 'setting llvm.link_shared is incompatible with download-ci-llvm.', config.rs:1010:17
Build completed unsuccessfully in 0:00:00
make: *** [Makefile:58: prepare] Error 1
Command failed. Attempt 5/5:
Building rustbuild
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.05s
thread 'main' panicked at 'setting llvm.link_shared is incompatible with download-ci-llvm.', config.rs:1010:17
Build completed unsuccessfully in 0:00:00
make: *** [Makefile:58: prepare] Error 1
The command has failed after 5 attempts.
