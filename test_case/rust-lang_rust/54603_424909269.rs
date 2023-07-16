plain
[00:41:38]    Compiling parking_lot_core v0.3.0
[00:41:38]    Compiling tempfile v3.0.3
[00:41:40]    Compiling parking_lot v0.6.4
[00:41:41]    Compiling rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[00:41:44] error[E0277]: `clean::get_path_for_type::AbsolutePathBuffer` doesn't implement `std::fmt::Debug`
[00:41:44]     --> librustdoc/clean/mod.rs:3974:9
[00:41:44]      |
[00:41:44] 3974 |     tcx.push_item_path(&mut apb, def_id);
[00:41:44]      |         ^^^^^^^^^^^^^^ `clean::get_path_for_type::AbsolutePathBuffer` cannot be formatted using `{:?}`
[00:41:44]      |
[00:41:44]      = help: the trait `std::fmt::Debug` is not implemented for `clean::get_path_for_type::AbsolutePathBuffer`
[00:41:44]      = note: add `#[derive(Debug)]` or manually implement `std::fmt::Debug`
[00:41:46] error: aborting due to previous error
[00:41:46] 
[00:41:46] For more information about this error, try `rustc --explain E0277`.
[00:41:46] error: Could not compile `rustdoc`.
---
[00:41:46] 
[00:41:46] 
[00:41:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:41:46] Build completed unsuccessfully in 0:37:02
[00:41:46] make: *** [all] Error 1
[00:41:46] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1a9c78a8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:03c636db:start=1538006626505453435,finish=1538006626509799862,duration=4346427
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:221e2be4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03e1a60a
travis_time:start:03e1a60a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6

