plain
[00:01:47] checking whether linker accepts -Wl,--gc-sections... yes
[00:01:47] checking whether linker accepts -Wl,--hash-style=both... yes
[00:01:47] checking whether linker accepts -Wl,--no-undefined... yes
[00:01:47] checking whether linker accepts -Wl,--exclude-libs=ALL... yes
[00:01:47] checking whether linker accepts -Wl,--dynamic-list=./dynamic.list... yes
[00:01:47] checking whether linker accepts -lgcc_eh... yes
[00:01:47] using compiler runtime libraries: -lgcc -lgcc_eh
[00:01:47] checking preprocessor condition __ILP32__... false
[00:01:47] checking whether compiler's long double definition matches float.h... yes
---
[00:02:20] Successfully tagged rust-ci:latest
[00:02:20] Built container sha256:c3cb2a1c700a72dde7037a67a69e86c949589b6b027667f1b5946acec0a07b0f
[00:02:20] Uploading finished image to s3://rust-lang-ci-sccache2/docker/28f55b5482325a293e12e0f8d7991c7106b2482afcd28cc84933fe48c3762e5a5db5abc971b2151de0ca286065d4cb418a24638eb6464d7658dc916276894637
[00:02:21] 
[00:02:21] Partial credentials found in env, missing: AWS_SECRET_ACCESS_KEY
[00:02:26] xargs: docker: terminated by signal 13

[00:02:26] travis_time:end:112c6d78:start=1539811715355308088,finish=1539811804951125525,duration=89595817437
[CI_JOB_NAME=dist-x86_64-musl]
[00:02:26] [CI_JOB_NAME=dist-x86_64-musl]
---

[00:25:29] travis_fold:start:stage1-rustc
travis_time:start:stage1-rustc
Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-musl)
[00:25:29] error: cannot produce dylib for `rustc_driver v0.0.0 (/checkout/src/librustc_driver)` as the target `x86_64-unknown-linux-musl` does not support these crate types
[00:25:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-musl" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:25:29] expected success, got: exit code: 101
[00:25:29] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:25:29] travis_fold:end:stage1-rustc

[00:25:29] travis_time:end:stage1-rustc:start=1539813188259260633,finish=1539813188661357202,duration=402096569

---
145436 ./obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu
145432 ./obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release
144588 ./obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/deps
135792 ./obj/build/bootstrap/debug/incremental/bootstrap-32pr67l4sa8g0
135788 ./obj/build/bootstrap/debug/incremental/bootstrap-32pr67l4sa8g0/s-f5tfso6g5m-maf97m-anymfpm2juvh
107664 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
10470
