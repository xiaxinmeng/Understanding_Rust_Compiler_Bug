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
    Finished release [optimized] target(s) in 14.73s
tidy check
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
tidy error: following path contains more than 939 entries, you should move the test to some relevant subdirectory (current: 940): /checkout/src/test/ui
* highest error code: E0791
* highest error code: E0791
tidy error: /checkout/compiler/rustc_feature/src/active.rs:419: no tracking issue for feature impl_restriction
tidy error: /checkout/compiler/rustc_feature/src/active.rs:455: no tracking issue for feature mut_restriction
tidy error: /checkout/compiler/rustc_resolve/src/build_reduced_graph.rs:859: TODO is deprecated; use FIXME
tidy error: /checkout/compiler/rustc_error_messages/src/lib.rs:71: line not in alphabetical order
Found 0 error(s) in error codes
Done!
some tidy checks failed
Build completed unsuccessfully in 0:00:19
