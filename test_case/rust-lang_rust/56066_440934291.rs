plain
travis_time:end:01f8a21a:start=1542869481046745718,finish=1542869535366293230,duration=54319547512
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:00:00] Attempting with retry: sh -c rm -f download-src-doc-book.tar.gz &&         curl -sSL -o download-src-doc-book.tar.gz https://github.com/rust-lang/book/archive/616fe4172b688393aeee5f34935cc25733c9c062.tar.gz
[00:00:00] rm 'src/doc/rust-by-example'
[00:00:00] Attempting with retry: sh -c rm -f download-src-doc-rust-by-example.tar.gz &&         curl -sSL -o download-src-doc-rust-by-example.tar.gz https://github.com/rust-lang/rust-by-example/archive/2ce92beabb912d417a7314d6da83ac9b50dc2afb.tar.gz
[00:00:00] rm 'src/llvm-emscripten'
[00:00:00] Attempting with retry: sh -c git submodule deinit -f  src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/stdsimd src/tools/lld src/libbacktrace src/tools/lldb src/tools/clang src/rust-sgx &&     git submodule sync &&     git submodule update -j 16 --init --recursive  src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/stdsimd src/tools/lld src/libbacktrace src/tools/lldb src/tools/clang src/rust-sgx
[00:00:00] Cleared directory 'src/dlmalloc'
[00:00:00] Cleared directory 'src/doc/nomicon'
[00:00:00] Cleared directory 'src/doc/reference'
[00:00:00] Cleared directory 'src/libbacktrace'
---
[00:00:00] Submodule 'src/doc/reference' (https://github.com/rust-lang-nursery/reference.git) registered for path 'src/doc/reference'
[00:00:00] Submodule 'src/libbacktrace' (https://github.com/rust-lang-nursery/libbacktrace.git) registered for path 'src/libbacktrace'
[00:00:00] Submodule 'src/libcompiler_builtins' (https://github.com/rust-lang-nursery/compiler-builtins.git) registered for path 'src/libcompiler_builtins'
[00:00:00] Submodule 'src/liblibc' (https://github.com/rust-lang/libc.git) registered for path 'src/liblibc'
[00:00:00] Submodule 'src/rust-sgx' (https://github.com/fortanix/rust-sgx) registered for path 'src/rust-sgx'
[00:00:00] Submodule 'src/tools/cargo' (https://github.com/rust-lang/cargo.git) registered for path 'src/tools/cargo'
[00:00:00] Submodule 'src/tools/clang' (https://github.com/rust-lang-nursery/clang.git) registered for path 'src/tools/clang'
[00:00:00] Submodule 'src/tools/clippy' (https://github.com/rust-lang-nursery/rust-clippy.git) registered for path 'src/tools/clippy'
[00:00:00] Submodule 'src/tools/lld' (https://github.com/rust-lang/lld.git) registered for path 'src/tools/lld'
[00:00:00] Submodule 'src/tools/lld' (https://github.com/rust-lang/lld.git) registered for path 'src/tools/lld'
[00:00:00] Submodule 'src/tools/lldb' (https://github.com/rust-lang-nursery/lldb.git) registered for path 'src/tools/lldb'
[00:00:00] Submodule 'src/tools/miri' (https://github.com/solson/miri.git) registered for path 'src/tools/miri'
[00:00:00] Submodule 'src/tools/rls' (https://github.com/rust-lang-nursery/rls.git) registered for path 'src/tools/rls'
[00:00:00] Submodule 'src/rust-installer' (https://github.com/rust-lang/rust-installer.git) registered for path 'src/tools/rust-installer'
[00:00:00] Submodule 'src/tools/rustfmt' (https://github.com/rust-lang-nursery/rustfmt.git) registered for path 'src/tools/rustfmt'
[00:00:00] Cloning into '/home/travis/build/rust-lang/rust/src/dlmalloc'...
[00:00:01] Cloning into '/home/travis/build/rust-lang/rust/src/rust-sgx'...
[00:00:01] Cloning into '/home/travis/build/rust-lang/rust/src/tools/rust-installer'...
[00:00:01] Cloning into '/home/travis/build/rust-lang/rust/src/doc/reference'...
[00:00:03] Cloning into '/home/travis/build/rust-lang/rust/src/libbacktrace'...
[00:00:03] Cloning into '/home/travis/build/rust-lang/rust/src/libcompiler_builtins'...
---
[00:00:51] Cloning into '/home/travis/build/rust-lang/rust/src/libcompiler_builtins/compiler-rt'...
[00:00:55] Cloning into '/home/travis/build/rust-lang/rust/src/libcompiler_builtins/libm'...
[00:00:55] Submodule path 'src/libcompiler_builtins/compiler-rt': checked out '7e387f0f90b493ae72930c787c381a80055a7ec9'
[00:00:55] Submodule path 'src/libcompiler_builtins/libm': checked out '3559e703795d33e84a91da2a35f2f3baac47e872'
[00:00:55] Submodule path 'src/liblibc': checked out '1eeba38558f5f83cd62901923f4bea8eea90bf82'
[00:00:55] Submodule path 'src/rust-sgx': checked out '11f01b5487aa6ddcd9829b1cfbe44be1a29517dc'
[00:00:55] Submodule path 'src/stdsimd': checked out '2f5e78c0b8bce40e2154764d63930741d7a65ca1'
[00:00:56] Submodule path 'src/tools/clang': checked out 'd0fc1788123de9844c8088b977cd142021cea1f2'
[00:00:56] Submodule path 'src/tools/clippy': checked out 'f5d868c9edfc6c2a9310d564a2f738bec67dfd6b'
[00:00:56] Submodule path 'src/tools/lld': checked out '2a9b88b8b419d094fb2185c0ca31c28d31bdca00'
[00:00:57] Submodule path 'src/tools/lldb': checked out 'fdea743be550ed8d7b61b2c908944cdd1290a6ad'
---
[00:20:53]    Compiling libc v0.0.0 (/checkout/src/rustc/libc_shim)
[00:20:53]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:20:54]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:21:00]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:21:02] error: trait objects without an explicit `dyn` are deprecated
[00:21:02]     |
[00:21:02]     |
[00:21:02] 489 |                 mem::transmute::<Box<FnBox()>, Box<FnBox()>>(Box::new(main))
[00:21:02]     |                                      ^^^^^^^ help: use `dyn`: `dyn FnBox()`
[00:21:02]     |
[00:21:02]     = note: requested on the command line with `-D bare-trait-objects`
[00:21:02] 
[00:21:02] error: trait objects without an explicit `dyn` are deprecated
[00:21:02]     |
[00:21:02]     |
[00:21:02] 489 |                 mem::transmute::<Box<FnBox()>, Box<FnBox()>>(Box::new(main))
[00:21:02]     |                                                    ^^^^^^^ help: use `dyn`: `dyn FnBox()`
[00:21:02] 
[00:21:02] error: trait objects without an explicit `dyn` are deprecated
[00:21:02]    |
[00:21:02]    |
[00:21:02] 46 |     fn write<T, F: FnOnce(&mut Write) -> T>(write: F);
[00:21:02]    |                                ^^^^^ help: use `dyn`: `dyn Write`
[00:21:02] 
[00:21:02] error: trait objects without an explicit `dyn` are deprecated
[00:21:02]    |
[00:21:02]    |
[00:21:02] 52 |     fn write<T, F: FnOnce(&mut Write) -> T>(_write: F) {
[00:21:02]    |                                ^^^^^ help: use `dyn`: `dyn Write`
[00:21:02] 
[00:21:02] error: trait objects without an explicit `dyn` are deprecated
[00:21:02]    |
[00:21:02]    |
[00:21:02] 62 |     fn write<T, F: FnOnce(&mut Write) -> T>(write: F) {
[00:21:02]    |                                ^^^^^ help: use `dyn`: `dyn Write`
0 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
33060 ./.git/modules/src/tools/lld
32672 ./.git/modules/src/tools/lld/objects
32664 ./.git/modules/src/tools/lld/objects/pack
---
travis_time:end:0a1527bb:start=1542870810102583361,finish=1542870810109816837,duration=7233476
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:35bade1a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1d7c9628
travis_time:start:1d7c9628
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.
