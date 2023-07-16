plain
[00:13:16]    Compiling rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:13:16]    Compiling rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:13:17]    Compiling rustc_allocator v0.0.0 (file:///checkout/src/librustc_allocator)
[00:15:04]    Compiling rustc_traits v0.0.0 (file:///checkout/src/librustc_traits)
[00:16:35] error: incorrect close delimiter: `)`
[00:16:35]    --> librustc_metadata/locator.rs:620:60
[00:16:35]     |
[00:16:35] 620 |                         warn!("no metadata found: {}", err));
[00:16:35]     |
[00:16:35] note: unclosed delimiter
[00:16:35]    --> librustc_metadata/locator.rs:619:33
[00:16:35]     |
[00:16:35]     |
[00:16:35] 619 |                     Err(err) => {
[00:16:35] 
[00:16:35] 
[00:16:35] error: unexpected close delimiter: `}`
[00:16:35]    --> librustc_metadata/locator.rs:838:1
[00:16:35] 838 | }
[00:16:35]     | ^
[00:16:35] 
[00:16:35] error: aborting due to 2 previous errors
[00:16:35] error: aborting due to 2 previous errors
[00:16:35] 
[00:16:35] error: Could not compile `rustc_metadata`.
[00:16:35] 
[00:16:35] Caused by:
[00:16:35]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_metadata librustc_metadata/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=915a4098329d68e9 -C extra-filename=-915a4098329d68e9 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-29fe21234b9fb4ff.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-c55d6c95192e4906.rlib --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-87919c9c23cf3652.so --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-09c0f3a89ad0d6b5.so --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-05301c67193a930e.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-f4ac364f854372fe.so --extern rustc_metadata_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_metadata_utils-9642bdbb3d859645.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-4a08b81d2b6640c1.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8d84add221c0f710.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8d84add221c0f710.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-0651ffc5a9129db1.so --extern syntax_ext=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_ext-d61f6c79c13e0eda.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-47b99ffec2efbd05.so -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-52504d5ed57fefc2/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-ee16f6821aef40e9/out` (exit code: 1)
[00:16:40] error: build failed
[00:16:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:16:40] expected success, got: exit code: 101
[00:16:40] expected success, got: exit code: 101
[00:16:40] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1155:9
[00:16:40] travis_fold:end:stage0-rustc

[00:16:40] travis_time:end:stage0-rustc:start=1536686740485535857,finish=1536687405847697596,duration=665362161739


[00:16:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:16:40] Build completed unsuccessfully in 0:12:00
[00:16:40] Makefile:28: recipe for target 'all' failed
[00:16:40] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05008752
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:03d108c1:start=1536687406685682288,finish=1536687406693322767,duration=7640479
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:008b501e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1ce9cdb8
travis_time:start:1ce9cdb8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0539c946
$ dmesg | grep -i kill
