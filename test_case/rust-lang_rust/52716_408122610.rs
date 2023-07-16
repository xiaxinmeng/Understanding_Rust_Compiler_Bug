plain
[00:00:00] Attempting with retry: sh -c rm -f download-src-doc-book.tar.gz &&         curl -sSL -o download-src-doc-book.tar.gz https://github.com/rust-lang/book/archive/f475da63a18d50217459a601cbef69a4bcac5e71.tar.gz
[00:00:00] rm 'src/doc/rust-by-example'
[00:00:00] Attempting with retry: sh -c rm -f download-src-doc-rust-by-example.tar.gz &&         curl -sSL -o download-src-doc-rust-by-example.tar.gz https://github.com/rust-lang/rust-by-example/archive/d2a64395a5210a61d3512a3a5c615f5c47699443.tar.gz
[00:00:00] rm 'src/llvm-emscripten'
[00:00:00] Attempting with retry: sh -c git submodule deinit -f  src/jemalloc src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/stdsimd src/tools/lld src/libbacktrace src/lldb src/clang &&     git submodule sync &&     git submodule update -j 16 --init --recursive  src/jemalloc src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/stdsimd src/tools/lld src/libbacktrace src/lldb src/clang
[00:00:00] Cleared directory 'src/clang'
[00:00:00] Cleared directory 'src/dlmalloc'
[00:00:00] Cleared directory 'src/doc/nomicon'
[00:00:00] Cleared directory 'src/doc/reference'
---
[00:00:00] Cleared directory 'src/tools/miri'
[00:00:00] Cleared directory 'src/tools/rls'
[00:00:00] Cleared directory 'src/tools/rust-installer'
[00:00:00] Cleared directory 'src/tools/rustfmt'
[00:00:00] Submodule 'src/clang' (https://github.com/rust-lang-nursery/clang/) registered for path 'src/clang'
[00:00:00] Submodule 'src/doc/nomicon' (https://github.com/rust-lang-nursery/nomicon.git) registered for path 'src/doc/nomicon'
[00:00:00] Submodule 'src/doc/reference' (https://github.com/rust-lang-nursery/reference.git) registered for path 'src/doc/reference'
[00:00:00] Submodule 'src/jemalloc' (https://github.com/rust-lang/jemalloc.git) registered for path 'src/jemalloc'
[00:00:00] Submodule 'src/libbacktrace' (https://github.com/rust-lang-nursery/libbacktrace) registered for path 'src/libbacktrace'
[00:00:00] Submodule 'src/libbacktrace' (https://github.com/rust-lang-nursery/libbacktrace) registered for path 'src/libbacktrace'
[00:00:00] Submodule 'src/libcompiler_builtins' (https://github.com/rust-lang-nursery/compiler-builtins) registered for path 'src/libcompiler_builtins'
[00:00:00] Submodule 'src/liblibc' (https://github.com/rust-lang/libc.git) registered for path 'src/liblibc'
[00:00:00] Submodule 'src/lldb' (https://github.com/rust-lang-nursery/lldb/) registered for path 'src/lldb'
[00:00:00] Submodule 'src/tools/cargo' (https://github.com/rust-lang/cargo.git) registered for path 'src/tools/cargo'
[00:00:00] Submodule 'src/tools/clippy' (https://github.com/rust-lang-nursery/rust-clippy.git) registered for path 'src/tools/clippy'
[00:00:00] Submodule 'src/tools/lld' (https://github.com/rust-lang/lld.git) registered for path 'src/tools/lld'
[00:00:00] Submodule 'src/tools/miri' (https://github.com/solson/miri.git) registered for path 'src/tools/miri'
---
[00:00:56] Cloning into '/home/travis/build/rust-lang/rust/src/tools/rustfmt'...
[00:00:56] Cloning into '/home/travis/build/rust-lang/rust/src/tools/cargo'...
[00:00:56] Cloning into '/home/travis/build/rust-lang/rust/src/liblibc'...
[00:00:56] Cloning into '/home/travis/build/rust-lang/rust/src/tools/lld'...
[00:00:56] Cloning into '/home/travis/build/rust-lang/rust/src/lldb'...
[00:00:57] Submodule path 'src/clang': checked out '6fda594059bd48b6b2ddcb34eda0a278aee2214e'
[00:00:57] Submodule path 'src/doc/nomicon': checked out '66ef7373409d1979c2839db8886ac2ec9b6a58cd'
[00:00:57] Submodule path 'src/doc/reference': checked out '0f63519ea10c028f48b2dbf7d0a2454203b68b0b'
[00:00:57] Submodule path 'src/jemalloc': checked out '1f5a28755e301ac581e2048011e4e0ff3da482ef'
[00:00:57] Submodule path 'src/libbacktrace': checked out 'f4d02bbdbf8a2c5a31f0801dfef597a86caad9e3'
---
[00:01:00] Cloning into '/home/travis/build/rust-lang/rust/src/libcompiler_builtins/libm'...
[00:01:01] Submodule path 'src/libcompiler_builtins/compiler-rt': checked out '40151c4c1cf77e593e3654e66e25ea423116aaae'
[00:01:01] Submodule path 'src/libcompiler_builtins/libm': checked out 'd65f60f24289ba212f5d47792f7236857efb2339'
[00:01:01] Submodule path 'src/liblibc': checked out 'b6d23ed45d72918239c0bfac11dc547895e59b81'
[00:01:02] Submodule path 'src/lldb': checked out '3dbe998969d457c5cef245f61b48bdaed0f5c059'
[00:01:02] Submodule path 'src/tools/cargo': checked out '506eea76edbf7198258265ddabcf320365bc4c5c'
[00:01:02] Submodule path 'src/tools/clippy': checked out 'afd91248eda02cf2968e4e02c77b6c10ecd3fd4f'
[00:01:02] Submodule path 'src/tools/lld': checked out '8214ccf861d538671b0a1436dbf4538dc4a64d09'
[00:01:02] Submodule path 'src/tools/miri': checked out '911aedf736992e907d11cb494167f41f28d02368'
---
[01:11:34] test builder::__test::test_with_no_doc_stage0 ... ok
[01:11:34] 
[01:11:34] failures:
[01:11:34] 
[01:11:34] ---- builder::__test::dist_baseline stdout ----
[01:11:34] thread 'builder::__test::dist_baseline' panicked at 'fs::read_dir(&libdir) failed with No such file or directory (os error 2)', bootstrap/dist.rs:2019:22
[01:11:34] 
[01:11:34] ---- builder::__test::dist_with_hosts stdout ----
[01:11:34] ---- builder::__test::dist_with_hosts stdout ----
[01:11:34] thread 'builder::__test::dist_with_hosts' panicked at 'fs::read_dir(&libdir) failed with No such file or directory (os error 2)', bootstrap/dist.rs:2019:22
[01:11:34] ---- builder::__test::dist_with_same_targets_and_hosts stdout ----
[01:11:34] ---- builder::__test::dist_with_same_targets_and_hosts stdout ----
[01:11:34] thread 'builder::__test::dist_with_same_targets_and_hosts' panicked at 'fs::read_dir(&libdir) failed with No such file or directory (os error 2)', bootstrap/dist.rs:2019:22
[01:11:34] ---- builder::__test::dist_with_targets stdout ----
[01:11:34] ---- builder::__test::dist_with_targets stdout ----
[01:11:34] thread 'builder::__test::dist_with_targets' panicked at 'fs::read_dir(&libdir) failed with No such file or directory (os error 2)', bootstrap/dist.rs:2019:22
[01:11:34] ---- builder::__test::dist_with_targets_and_hosts stdout ----
[01:11:34] ---- builder::__test::dist_with_targets_and_hosts stdout ----
[01:11:34] thread 'builder::__test::dist_with_targets_and_hosts' panicked at 'fs::read_dir(&libdir) failed with No such file or directory (os error 2)', bootstrap/dist.rs:2019:22
[01:11:34] 
[01:11:34] failures:
[01:11:34]     builder::__test::dist_baseline
[01:11:34]     builder::__test::dist_with_hosts
---
[01:11:34] 
[01:11:34] 
[01:11:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:11:34] Build completed unsuccessfully in 0:31:02
[01:11:34] make: *** [check] Error 1
[01:11:34] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:086a4ce6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:18dc0618:start=1532616399010251303,finish=1532616399018353654,duration=8102351
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:003880d8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:128cf20e
travis_time:start:128cf20e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01977450
$ dmesg | grep -i kill
