plain
[00:06:12]    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[00:06:17]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:06:21]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:06:25]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:37] error[E0277]: `parse::parser::TokenType` doesn't implement `std::fmt::Debug`
[00:06:37]     |
[00:06:37]     |
[00:06:37] 869 |                               kw.name(), self.expected_tokens));
[00:06:37]     |                                          ^^^^^^^^^^^^^^^^^^^^ `parse::parser::TokenType` cannot be formatted using `{:?}`
[00:06:37]     |
[00:06:37]     = help: the trait `std::fmt::Debug` is not implemented for `parse::parser::TokenType`
[00:06:37]     = note: add `#[derive(Debug)]` or manually implement `std::fmt::Debug`
[00:06:37]     = note: required because of the requirements on the impl of `std::fmt::Debug` for `std::vec::Vec<parse::parser::TokenType>`
[00:06:37]     = note: required by `std::fmt::Debug::fmt`
[00:06:40] error: aborting due to previous error
[00:06:40] 
[00:06:40] For more information about this error, try `rustc --explain E0277`.
[00:06:40] error: Could not compile `syntax`.
[00:06:40] error: Could not compile `syntax`.
[00:06:40] 
[00:06:40] To learn more, run the command again with --verbose.
[00:06:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:06:40] expected success, got: exit code: 101
[00:06:40] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:06:40] travis_fold:end:stage0-rustc

[00:06:40] travis_time:end:stage0-rustc:start=1538706624336085303,finish=1538706695486409537,duration=71150324234


[00:06:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:06:40] Build completed unsuccessfully in 0:02:12
[00:06:40] make: *** [all] Error 1
[00:06:40] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:021a1ace
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:24678e04:start=1538706696117094865,finish=1538706696120961547,duration=3866682
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04d7de77
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04986350
travis_time:start:04986350
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b799690
$ dmesg | grep -i kill
