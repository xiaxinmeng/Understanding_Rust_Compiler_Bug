
$ ./x.py test src/test/compile-fail --test-args feature-gate-more_struct_aliases

Finished debug [unoptimized] target(s) in 0.0 secs
Synchronizing submodule url for 'src/compiler-rt'
Synchronizing submodule url for 'src/jemalloc'
Synchronizing submodule url for 'src/liblibc'
Synchronizing submodule url for 'src/llvm'
Synchronizing submodule url for 'src/rt/hoedown'
Synchronizing submodule url for 'src/rust-installer'
HEAD is now at a8fc4c1 Merge pull request #28 from xen0n/preprocessor-firefighting
HEAD is now at e058ca6 Change how the default zone is found
HEAD is now at e49e9bb Auto merge of #478 - jackpot51:patch-1, r=alexcrichton
HEAD is now at ceb177e Merge pull request #60 from japaric/gh38406
HEAD is now at a3736a0 Merge pull request #6 from intelfx/patch-1
HEAD is now at 4f99485 Merge pull request #54 from brson/docdir
Building stage0 std artifacts (x86_64-apple-darwin -> x86_64-apple-darwin)
warning: /Library/Developer/CommandLineTools/usr/bin/ranlib: file: /Users/user/projects/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/build/compiler_builtins-0511dd5445a3a209/out/libcompiler-rt.a(addtf3.o) has no symbols
warning: /Library/Developer/CommandLineTools/usr/bin/ranlib: file: /Users/user/projects/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/build/compiler_builtins-0511dd5445a3a209/out/libcompiler-rt.a(divtf3.o) has no symbols
warning: /Library/Developer/CommandLineTools/usr/bin/ranlib: file: /Users/user/projects/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/build/compiler_builtins-0511dd5445a3a209/out/libcompiler-rt.a(multf3.o) has no symbols
warning: /Library/Developer/CommandLineTools/usr/bin/ranlib: file: /Users/user/projects/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/build/compiler_builtins-0511dd5445a3a209/out/libcompiler-rt.a(powitf2.o) has no symbols
warning: /Library/Developer/CommandLineTools/usr/bin/ranlib: file: /Users/user/projects/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/build/compiler_builtins-0511dd5445a3a209/out/libcompiler-rt.a(subtf3.o) has no symbols
warning: /Library/Developer/CommandLineTools/usr/bin/ranlib: file: /Users/user/projects/rust/build/x86_64-apple-darwin/stage0-std/x86_64-apple-darwin/release/build/compiler_builtins-0511dd5445a3a209/out/libcompiler-rt.a(trampoline_setup.o) has no symbols
    Finished release [optimized] target(s) in 0.0 secs
Copying stage0 std from stage0 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
Building stage0 test artifacts (x86_64-apple-darwin -> x86_64-apple-darwin)
    Finished release [optimized] target(s) in 0.0 secs
Copying stage0 test from stage0 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
Building stage0 compiler artifacts (x86_64-apple-darwin -> x86_64-apple-darwin)
warning: ../rustllvm/RustWrapper.cpp:151:3: warning: default label in switch which covers all enumeration values [-Wcovered-switch-default]
warning:   default:
warning:   ^
warning: ../rustllvm/RustWrapper.cpp:1235:3: warning: default label in switch which covers all enumeration values [-Wcovered-switch-default]
warning:   default:
warning:   ^
warning: ../rustllvm/RustWrapper.cpp:1268:3: warning: default label in switch which covers all enumeration values [-Wcovered-switch-default]
warning:   default:
warning:   ^
warning: ../rustllvm/RustWrapper.cpp:1282:3: warning: default label in switch which covers all enumeration values [-Wcovered-switch-default]
warning:   default:
warning:   ^
warning: 4 warnings generated.
    Finished release [optimized] target(s) in 0.2 secs
Copying stage0 rustc from stage0 (x86_64-apple-darwin -> x86_64-apple-darwin / x86_64-apple-darwin)
Copying stage1 compiler (x86_64-apple-darwin)
thread 'main' panicked at 'failed to copy `/Users/user/projects/rust/build/x86_64-apple-darwin/stage0-sysroot/lib/rustlib/x86_64-apple-darwin/lib/libflate-89345601b107c0b9.dylib` to `/Users/user/projects/rust/build/x86_64-apple-darwin/stage1/lib/libflate-89345601b107c0b9.dylib`: Permission denied (os error 13)', src/bootstrap/util.rs:53
note: Run with `RUST_BACKTRACE=1` for a backtrace.
