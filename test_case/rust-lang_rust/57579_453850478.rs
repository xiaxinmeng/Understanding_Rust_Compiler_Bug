plain
travis_time:end:0464b89f:start=1547397888245712305,finish=1547397960847756176,duration=72602043871
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:59:31]     Checking compiler_builtins v0.1.4
[00:59:33]  Documenting alloc v0.0.0 (/checkout/src/liballoc)
[00:59:38]     Finished release [optimized] target(s) in 39.66s
[00:59:38]  Documenting core v0.0.0 (/checkout/src/libcore)
[00:59:57] error: `[chain]` cannot be resolved, ignoring it...
[00:59:57]     |
[00:59:57]     |
[00:59:57] 431 | /// This is commonly used to adapt a single value generator into a [`chain`] of
[00:59:57]     |
[00:59:57] note: lint level defined here
[00:59:57]    --> src/libcore/lib.rs:64:9
[00:59:57]     |
[00:59:57]     |
[00:59:57] 64  | #![deny(intra_doc_link_resolution_failure)]
[00:59:57]     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:57]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:59:57] 
[00:59:57] error: `[chain]` cannot be resolved, ignoring it...
[00:59:57]     |
[00:59:57]     |
[00:59:57] 431 | /// This is commonly used to adapt a single value generator into a [`chain`] of
[00:59:57]     |
[00:59:57]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:59:57] 
[00:59:57] error: Could not document `core`.
[00:59:57] error: Could not document `core`.
[00:59:57] 
[00:59:57] Caused by:
[00:59:57]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name core src/libcore/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc --markdown-css rust.css --markdown-no-toc --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps` (exit code: 1)
[00:59:57] 
[00:59:57] 
[00:59:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-Z" "unstable-options" "-p" "core" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--index-page" "/checkout/src/doc/index.md"
[00:59:57] 
[00:59:57] 
[00:59:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:59:57] Build completed unsuccessfully in 0:05:30
[00:59:57] Build completed unsuccessfully in 0:05:30
[00:59:57] Makefile:18: recipe for target 'all' failed
[00:59:57] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:21181220
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Jan 13 17:46:07 UTC 2019
---
travis_time:end:14e1bba8:start=1547401568304317942,finish=1547401568313890284,duration=9572342
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06993a8f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f911f23
$ dmesg | grep -i kill
