plain
[00:07:18] For more information about this error, try `rustc --explain E0463`.
[00:07:18] error: Could not compile `rustdoc`.
[00:07:18] 
[00:07:18] Caused by:
[00:07:18]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustdoc librustdoc/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,metadata -C opt-level=2 -C metadata=60bcdc9d496c403c -C extra-filename=-60bcdc9d496c403c --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/release/deps --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libtempfile-449a781a9e9fafa1.rmeta --extern minifier=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libminifier-f7f1f75a773ab025.rmeta --extern pulldown_cmark=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libpulldown_cmark-fc247946a373a811.rmeta` (exit code: 101)
[00:07:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "--message-format" "json"
[00:07:18] expected success, got: exit code: 101
[00:07:18] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:07:18] travis_fold:end:stage0-rustdoc

[00:07:18] travis_time:end:stage0-rustdoc:start=1530308667568885844,finish=1530308676124850806,duration=8555964962

---
travis_time:end:0aa36c77:start=1530308676661917320,finish=1530308676668292089,duration=6374769
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:254d9eea
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04f4a570
$ dmesg | grep -i kill
