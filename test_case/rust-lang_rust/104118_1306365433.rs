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
   Compiling ignore v0.4.18
   Compiling xz2 v0.1.6
   Compiling toml v0.5.9
    Finished dev [unoptimized] target(s) in 21.10s
thread 'main' panicked at 'setting llvm.static_libstdcpp is incompatible with download-ci-llvm.', config.rs:1107:17
Build completed unsuccessfully in 0:00:38
make: *** [prepare] Error 1
Makefile:58: recipe for target 'prepare' failed
Command failed. Attempt 2/5:
Command failed. Attempt 2/5:
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.05s
thread 'main' panicked at 'setting llvm.static_libstdcpp is incompatible with download-ci-llvm.', config.rs:1107:17
Build completed unsuccessfully in 0:00:00
make: *** [prepare] Error 1
Makefile:58: recipe for target 'prepare' failed
Command failed. Attempt 3/5:
Command failed. Attempt 3/5:
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.06s
thread 'main' panicked at 'setting llvm.static_libstdcpp is incompatible with download-ci-llvm.', config.rs:1107:17
Build completed unsuccessfully in 0:00:00
make: *** [prepare] Error 1
Makefile:58: recipe for target 'prepare' failed
Command failed. Attempt 4/5:
Command failed. Attempt 4/5:
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.05s
thread 'main' panicked at 'setting llvm.static_libstdcpp is incompatible with download-ci-llvm.', config.rs:1107:17
Build completed unsuccessfully in 0:00:00
make: *** [prepare] Error 1
Makefile:58: recipe for target 'prepare' failed
Command failed. Attempt 5/5:
Command failed. Attempt 5/5:
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.05s
thread 'main' panicked at 'setting llvm.static_libstdcpp is incompatible with download-ci-llvm.', config.rs:1107:17
Build completed unsuccessfully in 0:00:00
make: *** [prepare] Error 1
Makefile:58: recipe for target 'prepare' failed
The command has failed after 5 attempts.
