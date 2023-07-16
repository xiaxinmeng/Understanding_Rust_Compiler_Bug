plain
[00:00:00] rm 'src/doc/rust-by-example'
[00:00:00] Attempting with retry: sh -c rm -f download-src-doc-rust-by-example.tar.gz &&         curl -sSL -o download-src-doc-rust-by-example.tar.gz https://github.com/rust-lang/rust-by-example/archive/e3719fc78ff4a21dfd13cfcc9e2ca42cb5de29f4.tar.gz
[00:00:00] rm 'src/llvm-emscripten'
[00:00:00] Attempting with retry: sh -c rm -f download-src-llvm-emscripten.tar.gz &&         curl -sSL -o download-src-llvm-emscripten.tar.gz https://github.com/rust-lang/llvm/archive/2717444753318e461e0c3b30dacd03ffbac96903.tar.gz
[00:00:00] Attempting with retry: sh -c git submodule deinit -f  src/jemalloc src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/stdsimd src/tools/lld src/libbacktrace src/lldb src/clang &&     git submodule sync &&     git submodule update -j 16 --init --recursive  src/jemalloc src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/stdsimd src/tools/lld src/libbacktrace src/lldb src/clang
[00:00:00] Cleared directory 'src/dlmalloc'
[00:00:00] Cleared directory 'src/doc/nomicon'
[00:00:00] Cleared directory 'src/doc/reference'
[00:00:00] Cleared directory 'src/jemalloc'
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
[00:01:05] Cloning into '/home/travis/build/rust-lang/rust/src/tools/rustfmt'...
[00:01:05] Cloning into '/home/travis/build/rust-lang/rust/src/liblibc'...
[00:01:05] Cloning into '/home/travis/build/rust-lang/rust/src/tools/cargo'...
[00:01:05] Cloning into '/home/travis/build/rust-lang/rust/src/tools/lld'...
[00:01:05] Cloning into '/home/travis/build/rust-lang/rust/src/lldb'...
[00:01:07] Submodule path 'src/clang': checked out '6fda594059bd48b6b2ddcb34eda0a278aee2214e'
[00:01:07] Submodule path 'src/doc/nomicon': checked out '790e96b87f4b5817cac310e73a524d25c3d076d8'
[00:01:07] Submodule path 'src/doc/reference': checked out '219e261ddb833a5683627b0a9be87a0f4486abb9'
[00:01:07] Submodule path 'src/jemalloc': checked out '1f5a28755e301ac581e2048011e4e0ff3da482ef'
[00:01:07] Submodule path 'src/libbacktrace': checked out 'f4d02bbdbf8a2c5a31f0801dfef597a86caad9e3'
---
[00:01:10] Cloning into '/home/travis/build/rust-lang/rust/src/libcompiler_builtins/libm'...
[00:01:10] Submodule path 'src/libcompiler_builtins/compiler-rt': checked out '40151c4c1cf77e593e3654e66e25ea423116aaae'
[00:01:11] Submodule path 'src/libcompiler_builtins/libm': checked out '96e36ea2620f9fbbaa46a01694a2fa3ef6c2fb7e'
[00:01:11] Submodule path 'src/liblibc': checked out 'b6d23ed45d72918239c0bfac11dc547895e59b81'
[00:01:11] Submodule path 'src/lldb': checked out '3dbe998969d457c5cef245f61b48bdaed0f5c059'
[00:01:12] Submodule path 'src/tools/cargo': checked out '2cd36b4ed1aef1ae39a30783e006411d1a4218ac'
[00:01:12] Submodule path 'src/tools/clippy': checked out 'b0dabce47803c18b935ec5390de69e04ad5304c2'
[00:01:12] Submodule path 'src/tools/lld': checked out '8214ccf861d538671b0a1436dbf4538dc4a64d09'
[00:01:12] Submodule path 'src/tools/miri': checked out 'e6f1e15676c26fdc7c4713647fe007b26f361a8e'
---
#######################################################################   99.8%
######################################################################## 100.0%
[00:02:15] extracting /checkout/obj/build/cache/2018-07-27/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:15]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:52] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:52] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:52] make: *** [prepare] Error 1
[00:02:52] Makefile:81: recipe for target 'prepare' failed
[00:02:53] Command failed. Attempt 2/5:
[00:02:53] Command failed. Attempt 2/5:
[00:02:53] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:53] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:53] Makefile:81: recipe for target 'prepare' failed
[00:02:53] make: *** [prepare] Error 1
[00:02:55] Command failed. Attempt 3/5:
[00:02:55] Command failed. Attempt 3/5:
[00:02:56] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:56] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:56] make: *** [prepare] Error 1
[00:02:56] Makefile:81: recipe for target 'prepare' failed
[00:02:59] Command failed. Attempt 4/5:
[00:02:59] Command failed. Attempt 4/5:
[00:02:59] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:59] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:59] Makefile:81: recipe for target 'prepare' failed
[00:02:59] make: *** [prepare] Error 1
[00:03:03] Command failed. Attempt 5/5:
[00:03:03] Command failed. Attempt 5/5:
[00:03:04] error: the lock file needs to be updated but --locked was passed to prevent this
[00:03:04] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:03:04] make: *** [prepare] Error 1
[00:03:04] Makefile:81: recipe for target 'prepare' failed
[00:03:04] The command has failed after 5 attempts.
travis_time:end:030cafce:start=1533156456624190948,finish=1533156640974884756,duration=184350693808
---
travis_time:end:064aaca7:start=1533156641475936033,finish=1533156641485818976,duration=9882943
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09b1f4f8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0621ab54
travis_time:start:0621ab54
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0c9afe24
$ dmesg | grep -i kill
