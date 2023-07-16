plain
travis_time:end:0ed7b1c0:start=1554804224967487192,finish=1554804227236758942,duration=2269271750
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:58:55]     Checking core v0.0.0 (/checkout/src/libcore)
[00:59:25]     Checking rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[00:59:25]     Checking compiler_builtins v0.1.9
[00:59:26]  Documenting alloc v0.0.0 (/checkout/src/liballoc)
[00:59:26] thread 'rustc' panicked at 'Table should have capacity at this point', src/libstd/collections/hash/table.rs:339:9
[00:59:26] 
[00:59:26] error: internal compiler error: unexpected panic
[00:59:26] 
[00:59:26] error: Unrecognized option: 'markdown-css'
[00:59:26] error: Unrecognized option: 'markdown-css'
[00:59:26] 
[00:59:26] error: Could not document `alloc`.
[00:59:26] 
[00:59:26] Caused by:
[00:59:26]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-name alloc src/liballoc/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc --markdown-css rust.css --markdown-no-toc --generate-redirect-pages --resource-suffix 1.35.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-983bfd746ea1187e.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-c94a89edb07c8a5f.rmeta` (exit code: 1)
[00:59:26] 
[00:59:26] 
[00:59:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-Z" "unstable-options" "-p" "alloc" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--generate-redirect-pages" "--resource-suffix" "1.35.0" "--index-page" "/checkout/src/doc/index.md"
[00:59:26] 
[00:59:26] 
[00:59:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:59:26] Build completed unsuccessfully in 0:07:18
---
travis_time:end:1849d1b8:start=1554807805892147928,finish=1554807805900113018,duration=7965090
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:065405b2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:29b71b40
$ dmesg | grep -i kill
