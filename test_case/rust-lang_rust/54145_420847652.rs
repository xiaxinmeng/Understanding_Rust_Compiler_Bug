plain
[00:20:25]    Compiling alloc_jemalloc v0.0.0 (file:///checkout/src/liballoc_jemalloc)
[00:20:28] error: build failed
[00:20:28] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:20:28] expected success, got: exit code: 101
[00:20:28] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1155:9
[00:20:28] travis_fold:end:stage1-std

[00:20:28] travis_time:end:stage1-std:start=1536800562278918855,finish=1536800572232211657,duration=9953292802


[00:20:28] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:20:28] Build completed unsuccessfully in 0:15:54
[00:20:28] make: *** [all] Error 1
[00:20:28] Makefile:28: recipe for target 'all' failed
1470244 ./obj
1470208 ./obj/build
1192184 ./.git
1067244 ./src
---
151412 ./src/tools/clang
149112 ./src/llvm-emscripten/test
149020 ./obj/build/bootstrap/debug/incremental
134588 ./obj/build/bootstrap/debug/incremental/bootstrap-j9sjo2qxwegl
134584 ./obj/build/bootstrap/debug/incremental/bootstrap-j9sjo2qxwegl/s-f4qzy5rdjf-1cauxoh-1jtu7ee7y4raz
104700 ./src/tools/lldb
98948 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
93748 ./src/tools/clang/test
93748 ./src/tools/clang/test
90752 ./obj/build/x86_64-unkn-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2b4191f8
$ dmesg | grep -i kill
