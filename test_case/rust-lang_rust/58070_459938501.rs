plain
travis_time:end:2811f027:start=1549083338745918743,finish=1549083339719494070,duration=973575327
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:57:28]     Checking core v0.0.0 (/checkout/src/libcore)
[00:57:57]     Checking rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[00:57:57]     Checking compiler_builtins v0.1.5
[00:57:59]  Documenting alloc v0.0.0 (/checkout/src/liballoc)
[00:57:59] error[E0602]: unknown lint: `missing_clone_implementations`
[00:57:59]   |
[00:57:59]   = help: did you mean: `missing_copy_implementations`
[00:57:59]   = note: requested on the command line with `-A missing_clone_implementations`
[00:58:00] error: Compilation failed, aborting rustdoc
[00:58:00] 
[00:58:00] error: Could not document `alloc`.
[00:58:00] 
[00:58:00] 
[00:58:00] Caused by:
[00:58:00]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name alloc src/liballoc/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc --markdown-css rust.css --markdown-no-toc --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-d54fe968dea87029.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-84e5b9599b1b7754.rmeta` (exit code: 1)
[00:58:00] 
[00:58:00] 
[00:58:00] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-Z" "unstable-options" "-p" "alloc" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--index-page" "/checkout/src/doc/index.md"
[00:58:00] 
[00:58:00] 
[00:58:00] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:58:00] Build completed unsuccessfully in 0:05:31
[00:58:00] Build completed unsuccessfully in 0:05:31
[00:58:00] make: *** [all] Error 1
[00:58:00] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:35d50f2e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Feb  2 05:53:52 UTC 2019
---
travis_time:end:0ecaee20:start=1549086833090839038,finish=1549086833096859706,duration=6020668
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09467123
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:117e1c24
travis_time:start:117e1c24
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2ba1ef0e
$ dmesg | grep -i kill
