plain
travis_time:end:332630e3:start=1541614101428998140,finish=1541614102570944219,duration=1141946079
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:48:15] ........................................................................................iiiii....... 900/5000
[00:48:18] .................................................................................................... 1000/5000
[00:48:20] .................................................................................................... 1100/5000
[00:48:23] .................................................................................................... 1200/5000
[00:48:25] .........................................FF.F....................................................... 1300/5000
[00:48:30] ...........................................................................................i........ 1500/5000
[00:48:33] .............................................................i...................................... 1600/5000
[00:48:36] .................................................................................................... 1700/5000
[00:48:40] .................................................................................................... 1800/5000
---
[00:49:05] .................................................................................................... 2500/5000
[00:49:09] .................................................................................................... 2600/5000
[00:49:13] .................................................................................................... 2700/5000
[00:49:16] .................................................................................................... 2800/5000
[00:49:19] ..................................................................................F................. 2900/5000
[00:49:26] ...........................i........................................................................ 3100/5000
[00:49:29] .........................................................................................i.i..ii.... 3200/5000
[00:49:32] .................................................................................................... 3300/5000
[00:49:36] .................................................................................................... 3400/5000
---
[00:50:22] 
[00:50:22] ---- [ui] ui/error-codes/E0719-trait-alias.rs stdout ----
[00:50:22] diff of stderr:
[00:50:22] 
[00:50:22] - error[E0719]: the value of the associated type `Item` (from the trait `std::iter::Iterator`) is already specified
[00:50:22] -   --> $DIR/E0719-trait-alias.rs:14:34
[00:50:22] + error[E0284]: type annotations required: cannot resolve `<Self as std::iter::I-- `Item` bound here first
[00:50:22] - ...
[00:50:22] - LL | trait U32Iterator2 = U32Iterator<Item = u32>; //~ ERROR E0719
[00:50:22] -    |                                  ^^^^^^^^^^ re-bound here
[00:50:22] + error: aborting due to previous error
[00:50:22] - error: aborting due to 3 previous errors
[00:50:22] - 
[00:50:22] - For more information about this error, try `rustc --explain E0719`.
[00:50:22] + For more information about this error, try `rustc --explain E0284`.
[00:50:22] + For more information about this error, try `rustc --explain E0284`.
[00:50:22] 30 
[00:50:22] 
[00:50:22] 
[00:50:22] The actual stderr differed from the expected stderr.
[00:50:22] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0719-trait-alias/E0719-trait-alias.stderr
[00:50:22] To update references, rerun the tests and pass the `--bless` flag
[00:50:22] To only update this specific test, also pass `--test-args error-codes/E0719-trait-alias.rs`
[00:50:22] error: 1 errors occurred comparing output.
[00:50:22] status: exit code: 1
[00:50:22] status: exit code: 1
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0719-trait-alias.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0719-trait-alias/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0719-trait-alias/auxiliary" "-A" "unused"
[00:50:22] ------------------------------------------
[00:50:22] 
[00:50:22] ------------------------------------------
[00:50:22] stderr:
[00:50:22] stderr:
[00:50:22] ------------------------------------------
[00:50:22] {"message":"type annotations required: cannot resolve `<Self as std::iter::Iterator>::Item == i32`","code":{"code":"E0284","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0719-trait-alias.rs","byte_start":659,"byte_end":704,"line_start":16,"line_end":16,"column_start":1,"column_end":46,"is_primary":true,"text":[{"text":"trait U32Iterator2 = U32Iterator<Item = u32>; //~ ERROR E0719","highlight_start":1,"highlight_end":46}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"required by `U32Iterator`","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0719-trait-alias.rs","byte_start":597,"byte_end":642,"line_start":15,"line_end":15,"column_start":1,"column_end":46,"is_primary":true,"text":[{"text":"trait U32Iterator = I32Iterator2<Item = i32>; //~ ERROR E0719","highlight_start":1,"highlight_end":46}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0284]: type annotations required: cannot resolve `<Self as std::iter::Iterator>::Item == i32`\n  --> /checkout/src/test/ui/error-codes/E0719-trait-alias.rs:16:1\n   |\nLL | trait U32Iterator2 = U32Iterator<Item = u32>; //~ ERROR E0719\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nnote: required by `U32Iterator`\n  --> /checkout/src/test/ui/error-codes/E0719-trait-alias.rs:15:1\n   |\nLL | trait U32Iterator = I32Iterator2<Item = i32>; //~ ERROR E0719\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:50:22] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:50:22] {"message":"For more information about this error, try `rustc --explain E0284`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0284`.\n"}
[00:50:22] ------------------------------------------
[00:50:22] 
[00:50:22] thread '[ui] ui/error-codes/E0719-trait-alias.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:50:22] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:50:22] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:50:22] 
[00:50:22] ---- [ui] ui/error-codes/E0719-trait-alias-object.rs stdout ----
[00:50:22] 
[00:50:22] error: ui test compiled successfully!
[00:50:22] status: exit code: 0
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0719-trait-alias-object.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0719-trait-alias-object/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0719-trait-alias-object/auxiliary" "-A" "unused"
[00:50:22] ------------------------------------------
[00:50:22] 
[00:50:22] ------------------------------------------
[00:50:22] stderr:
---
[00:50:22] ---- [ui] ui/error-codes/E0719.rs stdout ----
[00:50:22] 
[00:50:22] error: ui test compiled successfully!
[00:50:22] status: exit code: 0
[00:50:22] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0719.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0719/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0719/auxiliary" "-A" "unused"
[00:50:22] ------------------------------------------
[00:50:22] 
[00:50:22] ------------------------------------------
[00:50:22] stderr:
---
[00:50:22] 
[00:50:22] ---- [ui] ui/lint/issue-50589-multiple-associated-types.rs stdout ----
[00:50:22] diff of stderr:
[00:50:22] 
[00:50:22] - warning: associated type binding `Item` specified more than once
[00:50:22] -    |
[00:50:22] -    |
[00:50:22] - LL | fn test() ->  Box<Iterator<Item = (), Item = Unit>> {
[00:50:22] -    |                            ---------  ^^^^^^^^^^^ used more than once
[00:50:22] -    |                            first use of `Item`
[00:50:22] -    |
[00:50:22] -    |
[00:50:22] -    = note: #[warn(duplicate_associated_type_bindings)] on by default
[00:50:22] -    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:50:22] - 
[00:50:22] - 
[00:50:22] - warning: associated type binding `Item` specified more than once
[00:50:22] -    |
[00:50:22] -    |
[00:50:22] - LL | fn test() ->  Box<Iterator<Item = (), Item = Unit>> {
[00:50:22] -    |                            ---------  ^^^^^^^^^^^ used more than once
[00:50:22] -    |                            first use of `Item`
[00:50:22] -    |
[00:50:22] -    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:50:22] -    = note: for more information, see issue #50589 <https://github.com/rust-lang/rust/issues/50589>
[00:50:22] -    = note: for more information, see issue #50589 <https://github.com/rust-lang/rust/issues/50589>
[00:50:22] - 
[00:50:22] - 
[00:50:22] 
[00:50:22] 
[00:50:22] error: failed to delete `/checkout/obj/build/x86_64-unknown-linux-gnu/tenknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:50:22] 
[00:50:22] 
[00:50:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:50:22] Build completed unsuccessfully in 0:03:40
[00:50:22] Build completed unsuccessfully in 0:03:40
[00:50:22] make: *** [check] Error 1
[00:50:22] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:240c4da0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0aee8af4:start=1541617136840251653,finish=1541617136845788125,duration=5536472
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09353ee0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:074400de
travis_time:start:074400de
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2920518f
$ dmesg | grep -i kill
$ dmesg | grep -i kill
[   10.888543] init: failsafe main process (1093) killed by TERM signal
[   42.343723] init: plymouth-upstart-bridge main process (510) killed by TERM signal
travis_fold:end:after_failure.6

Done. Your build exited with 1.
