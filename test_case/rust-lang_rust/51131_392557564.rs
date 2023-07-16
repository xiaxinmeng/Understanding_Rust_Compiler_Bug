plain
[00:00:00] Submodule 'src/dlmalloc' (https://github.com/alexcrichton/dlmalloc-rs.git) registered for path 'src/dlmalloc'
[00:00:00] Submodule 'src/doc/nomicon' (https://github.com/rust-lang-nursery/nomicon.git) registered for path 'src/doc/nomicon'
[00:00:00] Submodule 'src/doc/reference' (https://github.com/rust-lang-nursery/reference.git) registered for path 'src/doc/reference'
[00:00:00] Submodule 'src/jemalloc' (https://github.com/rust-lang/jemalloc.git) registered for path 'src/jemalloc'
[00:00:00] Submodule 'src/libcompiler_builtins' (https://github.com/qnighy/compiler-builtins) registered for path 'src/libcompiler_builtins'
[00:00:00] Submodule 'src/stdsimd' (https://github.com/rust-lang-nursery/stdsimd) registered for path 'src/stdsimd'
[00:00:00] Submodule 'src/tools/cargo' (https://github.com/rust-lang/cargo.git) registered for path 'src/tools/cargo'
[00:00:00] Submodule 'src/tools/clippy' (https://github.com/rust-lang-nursery/rust-clippy.git) registered for path 'src/tools/clippy'
[00:00:00] Submodule 'src/tools/lld' (https://github.com/rust-lang/lld.git) registered for path 'src/tools/lld'
---
travis_time:start:tidy
tidy check
[00:04:55] * 547 error codes
[00:04:55] * highest error code: E0911
[00:04:55] tidy error: libsyntax/feature_gate.rs:480: no tracking issue for feature unsized_locals
[00:04:56] tidy error: libsyntax/feature_gate.rs:480: no tracking issue for feature unsized_locals
[00:04:56] some tidy checks failed
[00:04:56] 
[00:04:56] 
[00:04:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:56] 
[00:04:56] 
[00:04:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:56] Build completed unsuccessfully in 0:01:46
[00:04:56] Build completed unsuccessfully in 0:01:46
[00:04:56] Makefile:79: recipe for target 'tidy' failed
[00:04:56] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0806b926
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
