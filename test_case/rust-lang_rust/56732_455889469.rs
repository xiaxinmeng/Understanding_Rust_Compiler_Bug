plain
travis_time:end:0fbf2106:start=1548004425915832733,finish=1548004497689040427,duration=71773207694
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:46]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:04:02] warning: unnecessary `unsafe` block
[00:04:02]    --> src/libcore/num/mod.rs:71:30
[00:04:02]     |
[00:04:02] 33  | / macro_rules! nonzero_integers {
[00:04:02] 34  | |     ( $( $Ty: ident($Int: ty); )+ ) => {
[00:04:02] 36  | |             doc_comment! {
[00:04:02] ...   |
[00:04:02] 71  | |                         Some(unsafe { $Ty(n) })
[00:04:02]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:04:02]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:04:02] ...   |
[00:04:02] 97  | |     }
[00:04:02] 98  | | }
[00:04:02]     | |_- in this expansion of `nonzero_integers!`
[00:04:02] 100 | / nonzero_integers! {
[00:04:02] 101 | |     NonZeroU8(u8);
[00:04:02] 102 | |     NonZeroU16(u16);
[00:04:02] 103 | |     NonZeroU32(u32);
---
[00:04:02] 
[00:04:02] warning: unnecessary `unsafe` block
[00:04:02]    --> src/libcore/num/mod.rs:71:30
[00:04:02]     |
[00:04:02] 33  | / macro_rules! nonzero_integers {
[00:04:02] 34  | |     ( $( $Ty: ident($Int: ty); )+ ) => {
[00:04:02] 36  | |             doc_comment! {
[00:04:02] ...   |
[00:04:02] 71  | |                         Some(unsafe { $Ty(n) })
[00:04:02]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:04:02]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:04:02] ...   |
[00:04:02] 97  | |     }
[00:04:02] 98  | | }
[00:04:02]     | |_- in this expansion of `nonzero_integers!`
[00:04:02] 100 | / nonzero_integers! {
[00:04:02] 101 | |     NonZeroU8(u8);
[00:04:02] 102 | |     NonZeroU16(u16);
[00:04:02] 103 | |     NonZeroU32(u32);
---
[00:04:02] 
[00:04:02] warning: unused attribute
[00:04:02]    --> src/libcore/num/mod.rs:50:17
[00:04:02]     |
[00:04:02] 33  | / macro_rules! nonzero_integers {
[00:04:02] 34  | |     ( $( $Ty: ident($Int: ty); )+ ) => {
[00:04:02] 36  | |             doc_comment! {
[00:04:02] ...   |
[00:04:02] 50  | |                 #[rustc_layout_scalar_valid_range_start(1)]
[00:04:02]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:02]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:02] ...   |
[00:04:02] 97  | |     }
[00:04:02] 98  | | }
[00:04:02]     | |_- in this expansion of `nonzero_integers!`
[00:04:02] 100 | / nonzero_integers! {
[00:04:02] 101 | |     NonZeroU8(u8);
[00:04:02] 102 | |     NonZeroU16(u16);
[00:04:02] 103 | |     NonZeroU32(u32);
---
[00:04:02] 
[00:04:02] warning: unused attribute
[00:04:02]    --> src/libcore/num/mod.rs:50:17
[00:04:02]     |
[00:04:02] 33  | / macro_rules! nonzero_integers {
[00:04:02] 34  | |     ( $( $Ty: ident($Int: ty); )+ ) => {
[00:04:02] 36  | |             doc_comment! {
[00:04:02] ...   |
[00:04:02] 50  | |                 #[rustc_layout_scalar_valid_range_start(1)]
[00:04:02]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:02]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:02] ...   |
[00:04:02] 97  | |     }
[00:04:02] 98  | | }
[00:04:02]     | |_- in this expansion of `nonzero_integers!`
[00:04:02] 100 | / nonzero_integers! {
[00:04:02] 101 | |     NonZeroU8(u8);
[00:04:02] 102 | |     NonZeroU16(u16);
[00:04:02] 103 | |     NonZeroU32(u32);
---
[01:01:25]     Checking rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[01:01:25]     Checking rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[01:01:25]     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[01:01:26]  Documenting std v0.0.0 (/checkout/src/libstd)
[01:01:27] error[E0433]: failed to resolve: could not find `handle` in `sys`
[01:01:27]   --> src/libstd/sys/windows/ext/process.rs:13:27
[01:01:27]    |
[01:01:27] 13 |         let handle = sys::handle::Handle::new(handle as *mut _);
[01:01:27]    |                           ^^^^^^ could not find `handle` in `sys`
[01:01:27] 
[01:01:27] error[E0425]: cannot find function `symlink_inner` in module `sys::fs`
[01:01:27]    --> src/libstd/sys/windows/ext/fs.rs:476:14
[01:01:27]     |
[01:01:27] 476 |     sys::fs::symlink_inner(src.as_ref(), dst.as_ref(), false)
[01:01:27]     |              ^^^^^^^^^^^^^ not found in `sys::fs`
[01:01:27] 
[01:01:27] error[E0425]: cannot find function `symlink_inner` in module `sys::fs`
[01:01:27]    --> src/libstd/sys/windows/ext/fs.rs:497:14
[01:01:27]     |
[01:01:27] 497 |     sys::fs::symlink_inner(src.as_ref(), dst.as_ref(), true)
[01:01:27]     |              ^^^^^^^^^^^^^ not found in `sys::fs`
[01:01:27] error: Compilation failed, aborting rustdoc
[01:01:27] 
[01:01:27] error: aborting due to 4 previous errors
[01:01:27] 
[01:01:27] 
[01:01:27] Some errors occurred: E0425, E0433.
[01:01:27] For more information about an error, try `rustc --explain E0425`.
[01:01:27] error: Could not document `std`.
[01:01:27] 
[01:01:27] Caused by:
[01:01:27]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name std src/libstd/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc --cfg 'feature="backtrace"' --cfg 'feature="backtrace-sys"' --cfg 'feature="compiler_builtins"' --cfg 'feature="compiler_builtins_c"' --cfg 'feature="default"' --cfg 'feature="panic-unwind"' --cfg 'feature="panic_unwind"' --markdown-css rust.css --markdown-no-toc --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liballoc-af87d57ed062fc7f.rmeta --extern backtrace_sys=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libbacktrace_sys-c0930d7b9dbd7d13.rmeta --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-863701a3534445c1.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-f6a1ccb419e02477.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/liblibc-85aa98a45625ace7.rmeta --extern panic_abort=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_abort-a4efbc575de6bc74.rmeta --extern panic_unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libpanic_unwind-9f967884a5a8bc6a.rmeta --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-c4e32b3828a741b5.rmeta --extern rustc_asan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_asan-d66cb368c256ed0f.rmeta --extern rustc_lsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_lsan-2bb94db6f615f759.rmeta --extern rustc_msan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_msan-5735c5dc2cb08018.rmeta --extern rustc_tsan=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/librustc_tsan-be04ce754dd91366.rmeta --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libunwind-12e2a302e75e0a39.rmeta` (exit code: 1)
[01:01:27] 
[01:01:27] 
[01:01:27] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-Z" "unstable-options" "-p" "std" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--index-page" "/checkout/src/doc/index.md"
[01:01:27] 
[01:01:27] 
[01:01:27] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[01:01:27] Build completed unsuccessfully in 0:05:35
[01:01:27] Build completed unsuccessfully in 0:05:35
[01:01:27] make: *** [all] Error 1
[01:01:27] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:14bc6d08
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Jan 20 18:16:35 UTC 2019
---
travis_time:end:0b0ca4b8:start=1548008196153446452,finish=1548008196160370848,duration=6924396
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:157d6958
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0bfe1667
travis_time:start:0bfe1667
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0ee975b5
$ dmesg | grep -i kill
