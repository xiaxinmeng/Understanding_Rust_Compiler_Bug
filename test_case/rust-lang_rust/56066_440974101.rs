plain
travis_time:end:06106c62:start=1542877331669161870,finish=1542877334297038451,duration=2627876581
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
---
[00:00:01] Cloning into '/home/travis/build/rust-lang/rust/src/doc/nomicon'...
[00:00:01] Cloning into '/home/travis/build/rust-lang/rust/src/libcompiler_builtins'...
[00:00:01] Cloning into '/home/travis/build/rust-lang/rust/src/doc/reference'...
[00:00:03] Cloning into '/home/travis/build/rust-lang/rust/src/tools/rust-installer'...
[00:00:03] Cloning into '/home/travis/build/rust-lang/rust/src/rust-sgx'...
[00:00:03] Cloning into '/home/travis/build/rust-lang/rust/src/tools/miri'...
[00:00:03] Cloning into '/home/travis/build/rust-lang/rust/src/tools/rls'...
[00:00:03] Cloning into '/home/travis/build/rust-lang/rust/src/liblibc'...
[00:00:06] Cloning into '/home/travis/build/rust-lang/rust/src/stdsimd'...
---
[00:00:56] Cloning into '/home/travis/build/rust-lang/rust/src/libcompiler_builtins/compiler-rt'...
[00:01:00] Cloning into '/home/travis/build/rust-lang/rust/src/libcompiler_builtins/libm'...
[00:01:00] Submodule path 'src/libcompiler_builtins/compiler-rt': checked out '7e387f0f90b493ae72930c787c381a80055a7ec9'
[00:01:00] Submodule path 'src/libcompiler_builtins/libm': checked out '3559e703795d33e84a91da2a35f2f3baac47e872'
[00:01:00] Submodule path 'src/liblibc': checked out '1eeba38558f5f83cd62901923f4bea8eea90bf82'
[00:01:00] Submodule path 'src/rust-sgx': checked out '11f01b5487aa6ddcd9829b1cfbe44be1a29517dc'
[00:01:00] Submodule path 'src/stdsimd': checked out '2f5e78c0b8bce40e2154764d63930741d7a65ca1'
[00:01:01] Submodule path 'src/tools/clang': checked out 'd0fc1788123de9844c8088b977cd142021cea1f2'
[00:01:01] Submodule path 'src/tools/clippy': checked out 'f5d868c9edfc6c2a9310d564a2f738bec67dfd6b'
[00:01:01] Submodule path 'src/tools/lld': checked out '2a9b88b8b419d094fb2185c0ca31c28d31bdca00'
[00:01:02] Submodule path 'src/tools/lldb': checked out 'fdea743be550ed8d7b61b2c908944cdd1290a6ad'
---
[00:50:00]     Checking compiler_builtins v0.0.0 (/checkout/src/rustc/compiler_builtins_shim)
[00:50:02]  Documenting alloc v0.0.0 (/checkout/src/liballoc)
[00:50:07]     Finished release [optimized] target(s) in 44.03s
[00:50:07]  Documenting core v0.0.0 (/checkout/src/libcore)
[00:50:12] error: cannot find macro `call!` in this scope
[00:50:12]   --> libcore/../stdsimd/coresimd/arm/armclang.rs:58:25
[00:50:12]    |
[00:50:12] 58 |     constify_imm8!(val, call);
[00:50:12]    |
[00:50:12]    |
[00:50:12]    = help: have you added the `#[macro_use]` on the module/import?
[00:50:12] error: Could not document `core`.
[00:50:12] 
[00:50:12] Caused by:
[00:50:12] Caused by:
[00:50:12]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name core libcore/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc --markdown-css rust.css --markdown-no-toc --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 1)
[00:50:12] 
[00:50:12] 
[00:50:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-Z" "unstable-options" "-p" "core" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--index-page" "/checkout/src/doc/index.md"
[00:50:12] 
[00:50:12] 
[00:50:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:50:12] Build completed unsuccessfully in 0:05:13
[00:50:12] Build completed unsuccessfully in 0:05:13
[00:50:12] make: *** [all] Error 1
[00:50:12] Makefile:28: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:168c8fd0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Nov 22 09:52:36 UTC 2018
---
travis_time:end:055c79eb:start=1542880357717714764,finish=1542880357723101628,duration=5386864
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c091972
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then pr
