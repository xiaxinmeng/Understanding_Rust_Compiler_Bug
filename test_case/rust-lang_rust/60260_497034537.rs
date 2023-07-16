plain
travis_time:end:1b67b942:start=1559147213239536372,finish=1559147215391791526,duration=2152255154
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:05:08]     Checking rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[01:05:08]     Checking rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[01:05:08]     Checking hashbrown v0.3.0
[01:05:09]  Documenting std v0.0.0 (/checkout/src/libstd)
[01:05:10] error: macro expansion ignores token `;` and any following
[01:05:10]    --> src/libstd/sys/windows/c.rs:627:24
[01:05:10]     |
[01:05:10] 612 | / ifdef! {
[01:05:10] 613 | |     pub const EXCEPTION_CONTINUE_SEARCH: LONG = 0;
[01:05:10] 614 | |     pub const EXCEPTION_STACK_OVERFLOW: DWORD = 0xc00000fd;
[01:05:10] 615 | |     pub const EXCEPTION_MAXIMUM_PARAMETERS: usize = 15;
[01:05:10] 627 | |     pub enum CONTEXT {};
[01:05:10]     | |                        ^
[01:05:10] ...   |
[01:05:10] 708 | |     }
[01:05:10] 708 | |     }
[01:05:10] 709 | | }
[01:05:10]     | |_- caused by the macro expansion here
[01:05:10]     |
[01:05:10]     = note: the usage of `ifdef!` is likely invalid in item context
[01:05:10] error: Compilation failed, aborting rustdoc
[01:05:10] 
[01:05:10] error: aborting due to 2 previous errors
[01:05:10] 
[01:05:10] 
[01:05:10] error: Could not document `std`.
[01:05:10] 
[01:05:10] Caused by:
[01:05:10]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-name std src/libstd/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="alloc"' --cfg 'feature="backtrace"' --cfg 'feature="compiler-builtins-c"' --cfg 'feature="default"' --cfg 'feature="panic-unwind"' --cfg 'feature="panic_unwind"' --cfg 'feature="std_detect_dlsym_getauxval"' --cfg 'feature="std_detect_file_io"' --markdown-css rust.css --markdown-no-toc --generate-redirect-pages --resource-suffix 1.37.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc-f13b5048eea2d168.rmeta --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libbacktrace-472cf631f10c28a2.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-651c3f0115992c57.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-0ccabe5d6e08a700.rmeta --extern hashbrown=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libhashbrown-e3d2d122f8775782.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liblibc-b9935d582e289c41.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-53aa1beee8d7b2eb.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-4ecf84ccd38afda1.rmeta --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-55d35b7ccafeb0d0.rmeta --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-5e93bb504bafb5cd.rmeta --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-b4950d328651cdfc.rmeta --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-901eb684803e83c4.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libunwind-ccb68891546b997c.rmeta` (exit code: 1)
[01:05:10] 
[01:05:10] 
[01:05:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-Z" "unstable-options" "-p" "std" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--generate-redirect-pages" "--resource-suffix" "1.37.0" "--index-page" "/checkout/src/doc/index.md"
[01:05:10] 
[01:05:10] 
[01:05:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[01:05:10] Build completed unsuccessfully in 0:07:48
[01:05:10] Build completed unsuccessfully in 0:07:48
[01:05:10] Makefile:18: recipe for target 'all' failed
[01:05:10] make: *** [all] Error 1
./src/llvm-project/clang
150716 ./obj/build/bootstrap/debug/incremental
142904 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release
135432 ./obj/build/bootstrap/debug/incremental/bootstrap-oxgzobynhmm1
135428 ./obj/build/bootstrap/debug/incremental/bootstrap-oxgzobynhmm1/s-fco6f9xixg-11asn1e-2xf11oh8rvch9
112028 ./obj/build/x86_64-unknown-linux-gnu/doc/core
108532 ./src/llvm-project/lldb
107532 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc
102232 ./.git
---
61180 ./obj/build/e:end:237965ec:start=1559151138370103536,finish=1559151138375065267,duration=4961731
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b7188ca
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1b58db0b
travis_time:start:1b58db0b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0eb6dd05
$ dmesg | grep -i kill
