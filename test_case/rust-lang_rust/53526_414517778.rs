plain
[00:17:52]    Compiling rustc_passes v0.0.0 (file:///checkout/src/librustc_passes)
tax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-64bbe8e4870170a3.so --extern syntax_ext=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_ext-29dfa988647839fa.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-3b51f50aecba154c.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-6c2cab36846647d7/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-3c585aa15bfc4e69/out` (exit code: 1)
[00:18:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:18:19] expected success, got: exit code: 101
[00:18:19] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:18:19] travis_fold:end:stage0-rustc

[00:18:19] travis_time:end:stage0-rustc:start=1534813500527169022,finish=1534814260313494257,duration=759786325235


[00:18:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:18:19] Build completed unsuccessfully in 0:13:33
[00:18:19] make: *** [all] Error 1
[00:18:19] Makefile:28: recipe for target 'all' failed
travis_time:end:153636260 .
1400088 ./obj/build
1177028 ./.git
1058596 ./src
794336 ./obj/build/x86_64-unknown-linux-gnu
---
149120 ./src/llvm-emscripten/test
148688 ./obj/build/bootstrap/debug/incremental
140868 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
134256 ./obj/build/bootstrap/debug/incremental/bootstrap-1v3ifugz4t07z
134252 ./obj/build/bootstrap/debug/incremental/bootstrap-1v3ifugz4t07z/s-f41nic04cw-vjejem-1iplbejydu42a
103868 ./src/tools/lldb
102552 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
102548 ./obj/build/x8tins/modules
34916 ./obj/build/x86_64-unknown-linux-gnu/native/jemalloc/src
---
21116 ./.git/modules/src/tools/cargo/objects/pack
travis_time:end:0c776fcb:start=1534814260624132380,finish=1534814260972190967,duration=348058587
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
trav || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0d27c89c
$ dmesg | grep -i kill
