plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:00159484
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[01:46:17]    Compiling humantime v1.1.1
[01:46:17]    Compiling textwrap v0.10.0
[01:46:19]    Compiling aho-corasick v0.6.7
[01:46:20]    Compiling libssh2-sys v0.2.10
[01:46:22] error[E0277]: the trait bound `T: de::value::private::Pair` is not satisfied
[01:46:22]     --> /cargo/registry/src/github.com-1ecc6299db9ec823/serde-1.0.75/src/de/value.rs:1393:5
[01:46:22]      |
[01:46:22] 1393 |     pub type First<T> = <T as Pair>::First;
[01:46:22]      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `de::value::private::Pair` is not implemented for `T`
[01:46:22]      |
[01:46:22]      = help: consider adding a `where T: de::value::private::Pair` bound
[01:46:22] 
[01:46:22] error[E0277]: the trait bound `T: de::value::private::Pair` is not satisfied
[01:46:22]     --> /cargo/registry/src/github.com-1ecc6299db9ec823/serde-1.0.75/src/de/value.rs:1394:5
[01:46:22]      |
[01:46:22] 1394 |     pub type Second<T> = <T as Pair>::Second;
[01:46:22]      |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `de::value::private::Pair` is not implemented for `T`
[01:46:22]      |
[01:46:22]      = help: consider adding a `where T: de::value::private::Pair` bound
[01:46:24] error: aborting due to 2 previous errors
[01:46:24] 
[01:46:24] For more information about this error, try `rustc --explain E0277`.
[01:46:24] error: Could not compile `serde`.
[01:46:24] error: Could not compile `serde`.
[01:46:24] 
[01:46:24] Caused by:
[01:46:24]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name serde /cargo/registry/src/github.com-1ecc6299db9ec823/serde-1.0.75/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 --cfg 'feature="default"' --cfg 'feature="std"' -C metadata=b65eee74cc76710b -C extra-filename=-b65eee74cc76710b --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -C linker=clang -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps --cap-lints allow --cfg de_boxed_c_str --cfg de_rc_dst --cfg core_duration --cfg integer128 --cfg range_inclusive --cfg num_nonzero` (exit code: 1)
[01:46:25] error: build failed
[01:46:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/cargo/Cargo.toml" "--features" "" "--message-format" "json"
[01:46:25] expected success, got: exit code: 101
[01:46:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
---
travis_time:end:02c759c8:start=1536364691500559974,finish=1536364691509015229,duration=8455255
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:19d8a8c3
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:039ddf0e
travis_time:start:039ddf0e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0126a2fa
$ dmesg | grep -i kill
