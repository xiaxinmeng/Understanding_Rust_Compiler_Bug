\n\nAn `if` expression without an `else` block has the type `()`, so this is a type\nerror. To resolve it, add an `else` block having the same type as the `if`\nblock.\n"},"level":"error","spans":[{"file_name":"<::core::macros::assert_eq macros>","byte_start":105,"byte_end":258,"line_start":5,"line_end":10,"column_start":1,"column_end":29,"is_primary":true,"text":[{"text":"if ! ( * left_val == * right_val ) {","highlight_start":1,"highlight_end":37},{"text":"panic ! (","highlight_start":1,"highlight_end":10},{"text":"r#\"assertion failed: `(left == right)`","highlight_start":1,"highlight_end":39},{"text":"  left: `{:?}`,","highlight_start":1,"highlight_end":16},{"text":" right: `{:?}`\"# , & *","highlight_start":1,"highlight_end":23},{"text":"left_val , & * right_val ) } } } } ) ; ( $ left : expr , $ right : expr , ) =>","highlight_start":1,"highlight_end":29}],"label":"expected (), found isize","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/issues/issue-50577.rs","byte_start":42,"byte_end":58,"line_start":3,"line_end":3,"column_start":16,"column_end":32,"is_primary":false,"text":[{"text":"        Drop = assert_eq!(1, 1)","highlight_start":16,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"assert_eq!","def_site_span":{"file_name":"<::core::macros::assert_eq macros>","byte_start":0,"byte_end":687,"line_start":1,"line_end":21,"column_start":1,"column_end":77,"is_primary":false,"text":[{"text":"( $ left : expr , $ right : expr ) => (","highlight_start":1,"highlight_end":40},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"match ( & $ left , & $ right ) {","highlight_start":1,"highlight_end":33},{"text":"( left_val , right_val ) => {","highlight_start":1,"highlight_end":30},{"text":"if ! ( * left_val == * right_val ) {","highlight_start":1,"highlight_end":37},{"text":"panic ! (","highlight_start":1,"highlight_end":10},{"text":"r#\"assertion failed: `(left == right)`","highlight_start":1,"highlight_end":39},{"text":"  left: `{:?}`,","highlight_start":1,"highlight_end":16},{"text":" right: `{:?}`\"# , & *","highlight_start":1,"highlight_end":23},{"text":"left_val , & * right_val ) } } } } ) ; ( $ left : expr , $ right : expr , ) =>","highlight_start":1,"highlight_end":79},{"text":"( { assert_eq ! ( $ left , $ right ) } ) ; (","highlight_start":1,"highlight_end":45},{"text":"$ left : expr , $ right : expr , $ ( $ arg : tt ) + ) => (","highlight_start":1,"highlight_end":59},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"match ( & ( $ left ) , & ( $ right ) ) {","highlight_start":1,"highlight_end":41},{"text":"( left_val , right_val ) => {","highlight_start":1,"highlight_end":30},{"text":"if ! ( * left_val == * right_val ) {","highlight_start":1,"highlight_end":37},{"text":"panic ! (","highlight_start":1,"highlight_end":10},{"text":"r#\"assertion failed: `(left == right)`","highlight_start":1,"highlight_end":39},{"text":"  left: `{:?}`,","highlight_start":1,"highlight_end":16},{"text":" right: `{:?}`: {}\"# ,","highlight_start":1,"highlight_end":23},{"text":"& * left_val , & * right_val , format_args ! ( $ ( $ arg ) + ) ) } } } } ) ;","highlight_start":1,"highlight_end":77}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"expected type `()`\n   found type `isize`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0317]: if may be missing an else clause\n  --> /checkout/src/test/ui/issues/issue-50577.rs:3:16\n   |\nLL |         Drop = assert_eq!(1, 1)\n   |                ^^^^^^^^^^^^^^^^ expected (), found isize\n   |\n   = note: expected type `()`\n              found type `isize`\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[01:14:46] {"message":"For more information about this error, try `rustc --explain E0317`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0317`.\n"}
[01:14:46] 
[01:14:46] ------------------------------------------
[01:14:46] 
---
[01:14:46] 
[01:14:46] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:14:46] 
[01:14:46] 
[01:14:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:14:46] 
[01:14:46] 
[01:14:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:14:46] Build completed unsuccessfully in 0:04:24
[01:14:46] Build completed unsuccessfully in 0:04:24
[01:14:46] make: *** [check] Error 1
[01:14:46] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02f39c8b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Mar 28 14:08:17 UTC 2019
---
travis_time:end:00bfa765:start=1553782099092387470,finish=1553782099097580423,duration=5192953
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:116ba142
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0dbbaf21
travis_time:start:0dbbaf21
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06318674
$ dmesg | grep -i kill
