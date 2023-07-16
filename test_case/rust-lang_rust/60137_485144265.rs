plain
travis_time:end:0a3d1850:start=1555776723440280871,finish=1555776812281563059,duration=88841282188
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:01:08]     Checking core v0.0.0 (/checkout/src/libcore)
[01:01:39]     Checking rustc-std-workspace-core v1.0.0 (/checkout/src/tools/rustc-std-workspace-core)
[01:01:39]     Checking compiler_builtins v0.1.10
[01:01:40]  Documenting alloc v0.0.0 (/checkout/src/liballoc)
[01:01:43] error: internal compiler error: src/librustc/hir/def.rs:287: attempted .def_id() on invalid def: Err
[01:01:43] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:636:9
[01:01:43] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:01:44] error: aborting due to previous error
[01:01:44] 
[01:01:44] 
[01:01:44] 
[01:01:44] error: Unrecognized option: 'markdown-css'
[01:01:44] 
[01:01:44] error: Could not document `alloc`.
[01:01:44] 
[01:01:44] Caused by:
[01:01:44]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-name alloc src/liballoc/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/doc --markdown-css rust.css --markdown-no-toc --generate-redirect-pages --resource-suffix 1.36.0 --index-page /checkout/src/doc/index.md -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcompiler_builtins-c48a38dddb046cd4.rmeta --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps/libcore-72f00ef37db6d7ed.rmeta` (exit code: 1)
[01:01:44] 
[01:01:44] 
[01:01:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-Z" "unstable-options" "-p" "alloc" "--" "--markdown-css" "rust.css" "--markdown-no-toc" "--generate-redirect-pages" "--resource-suffix" "1.36.0" "--index-page" "/checkout/src/doc/index.md"
[01:01:44] 
[01:01:44] 
[01:01:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[01:01:44] Build completed unsuccessfully in 0:07:40
