plain
[00:08:21]     Checking parking_lot_core v0.2.14
[00:08:21]     Checking parking_lot v0.5.5
[00:08:22]     Checking tempfile v3.0.3
[00:08:22]     Checking rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[00:08:28] error[E0597]: borrowed value does not live long enough
[00:08:28]    --> librustdoc/clean/inline.rs:541:8
[00:08:28]     |
[00:08:28] 541 |     if cx.external_traits.lock().borrow().contains_key(&did) ||
[00:08:28]     |        ^^^^^^^^^^^^^^^^^^^^^^^^^ temporary value does not live long enough
[00:08:28] 542 |         cx.active_extern_traits.borrow().contains(&did)
[00:08:28]     |                                                       - temporary value dropped here while still borrowed
[00:08:28]     |
[00:08:28]     = note: values in a scope are dropped in the opposite order they are created
[00:08:28] 
[00:08:28] error[E0597]: borrowed value does not live long enough
[00:08:28]    --> librustdoc/clean/inline.rs:552:5
[00:08:28]     |
[00:08:28] 552 |     cx.external_traits.lock().borrow_mut().insert(did, trait_);
[00:08:28]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^                                 - temporary value dropped here while still borrowed
[00:08:28]     |     |
[00:08:28]     |     temporary value does not live long enough
[00:08:28]     |
[00:08:28]     = note: values in a scope are dropped in the opposite order they are created
[00:08:28]     = note: consider using a `let` binding to increase its lifetime
[00:08:29] error: aborting due to 2 previous errors
[00:08:29] 
[00:08:29] For more information about this error, try `rustc --explain E0597`.
[00:08:29] error: Could not compile `rustdoc`.
[00:08:29] error: Could not compile `rustdoc`.
[00:08:29] 
[00:08:29] Caused by:
[00:08:29]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustdoc librustdoc/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,metadata -C opt-level=2 -C metadata=4484923db6e15805 -C extra-filename=-4484923db6e15805 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/release/deps --extern minifier=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libminifier-fab1f66c59304371.rmeta --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-78b3b548d9e00af4.rmeta --extern pulldown_cmark=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libpulldown_cmark-d0b1f79e5ad6aec4.rmeta --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/libtempfile-b20a480e56610e05.rmeta` (exit code: 1)
[00:08:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "--message-format" "json"
[00:08:29] expected success, got: exit code: 101
[00:08:29] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1155:9
[00:08:29] travis_fold:end:stage0-rustdoc

[00:08:29] travis_time:end:stage0-rustdoc:start=1536197172812203979,finish=1536197183651266830,duration=10839062851

---
travis_time:end:02fcaab8:start=1536197184472021295,finish=1536197184479098320,duration=7077025
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02a621dc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:16cfe706
travis_time:start:16cfe706
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00fcbf2e
$ dmesg | grep -i kill
