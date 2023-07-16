plain
[00:07:07] 
[00:07:07] error[E0433]: failed to resolve. Use of undeclared type or module `Edition`
[00:07:07]     --> librustc_lint/builtin.rs:1928:30
[00:07:07]      |
[00:07:07] 1928 |                     "try" => Edition::Edition2018,
[00:07:07]      |                              ^^^^^^^ Use of undeclared type or module `Edition`
[00:07:08] error: aborting due to 2 previous errors
[00:07:08] 
[00:07:08] For more information about this error, try `rustc --explain E0433`.
[00:07:08] error: Could not compile `rustc_lint`.
[00:07:08] error: Could not compile `rustc_lint`.
[00:07:08] 
[00:07:08] Caused by:
[00:07:08]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_lint librustc_lint/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,metadata -C prefer-dynamic -C opt-level=2 -C metadata=ca229e70e4528598 -C extra-filename=-ca229e70e4528598 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-445c78b30a600938.rmeta --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-c80832497be56dad.rmeta --extern rustc_mir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-d8b9538784647156.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-62a91fdc5731dc0e.rmeta --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-cb7790fd9ee13ed4.rmeta --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-6449186865dade13.rmeta -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-ee16f6821aef40e9/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-52504d5ed57fefc2/out` (exit code: 1)
[00:07:09] error: build failed
[00:07:09] error: build failed
[00:07:09] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:07:09] expected success, got: exit code: 101
[00:07:09] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:07:09] travis_fold:end:stage0-rustc

[00:07:09] travis_time:end:stage0-rustc:start=1535531902853676821,finish=1535532057503086294,duration=154649409473

---
travis_time:end:0172f4ca:start=1535532058235022235,finish=1535532058241271345,duration=6249110
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ee0f50e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0087bb3e
travis_time:start:0087bb3e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02267b18
$ dmesg | grep -i kill
