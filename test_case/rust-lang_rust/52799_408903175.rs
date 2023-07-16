plain
[00:06:06] 
[00:06:06] error: Could not compile `rustc_data_structures`.
[00:06:06] 
[00:06:06] Caused by:
[00:06:06]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_data_structures librustc_data_structures/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=3d8079c0b5c752f1 -C extra-filename=-3d8079c0b5c752f1 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libcfg_if-f1848c30fab0ba2e.rlib --extern ena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libena-52cd1efcc942bacf.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-eaf85f66e467efb7.rlib --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-c2d75c4ffd469ddc.rlib --extern parking_lot_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot_core-6ddf17f910242635.rlib --extern rustc_hash=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hash-50d2fcfa7f0ff0df.rlib --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-c7c8f17fb8a8edba.rlib --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-18ba5ff3e9db22d8.rlib --extern rustc_cratesio_shim=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_cratesio_shim-ff1c9259a238aca6.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-a86c1222f09eecc5.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-a86c1222f09eecc5.rlib --extern stable_deref_trait=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libstable_deref_trait-fbfca448f722407d.rlib` (exit code: 101)
[00:06:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:06:06] expected success, got: exit code: 101
[00:06:06] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:06:06] travis_fold:end:stage0-rustc

[00:06:06] travis_time:end:stage0-rustc:start=1532964230585699432,finish=1532964278296806587,duration=47711107155


[00:06:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:06:06] Build completed unsuccessfully in 0:01:49
[00:06:06] make: *** [all] Error 1
[00:06:06] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1aefeb64
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0189a240:start=1532964278836569574,finish=1532964278843143740,duration=6574166
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06c9bc68
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08e5c3fe
travis_time:start:08e5c3fe
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1d789fca
