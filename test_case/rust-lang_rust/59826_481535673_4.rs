\n\nRust only looks at the signature of the called function, as such it must\nalready specify all requirements that will be used for every type parameter.\n"},"level":"error","spans":[{"file_name":"<::core::macros::assert_eq macros>","byte_start":123,"byte_end":125,"line_start":5,"line_end":5,"column_start":19,"column_end":21,"is_primary":true,"text":[{"text":"if ! ( * left_val == * right_val ) {","highlight_start":19,"highlight_end":21}],"label":"no implementation for `&str == (&str,)`","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs","byte_start":1436,"byte_end":1471,"line_start":59,"line_end":59,"column_start":5,"column_end":40,"is_primary":false,"text":[{"text":"    assert_eq!(\"Yeah\", dbg!(\"Yeah\", ));","highlight_start":5,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"assert_eq!","def_site_span":{"file_name":"<::core::macros::assert_eq macros>","byte_start":0,"byte_end":687,"line_start":1,"line_end":21,"column_start":1,"column_end":77,"is_primary":false,"text":[{"text":"( $ left : expr , $ right : expr ) => (","highlight_start":1,"highlight_end":40},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"match ( & $ left , & $ right ) {","highlight_start":1,"highlight_end":33},{"text":"( left_val , right_val ) => {","highlight_start":1,"highlight_end":30},{"text":"if ! ( * left_val == * right_val ) {","highlight_start":1,"highlight_end":37},{"text":"panic ! (","highlight_start":1,"highlight_end":10},{"text":"r#\"assertion failed: `(left == right)`","highlight_start":1,"highlight_end":39},{"text":"  left: `{:?}`,","highlight_start":1,"highlight_end":16},{"text":" right: `{:?}`\"# , & *","highlight_start":1,"highlight_end":23},{"text":"left_val , & * right_val ) } } } } ) ; ( $ left : expr , $ right : expr , ) =>","highlight_start":1,"highlight_end":79},{"text":"( { assert_eq ! ( $ left , $ right ) } ) ; (","highlight_start":1,"highlight_end":45},{"text":"$ left : expr , $ right : expr , $ ( $ arg : tt ) + ) => (","highlight_start":1,"highlight_end":59},{"text":"{","highlight_start":1,"highlight_end":2},{"text":"match ( & ( $ left ) , & ( $ right ) ) {","highlight_start":1,"highlight_end":41},{"text":"( left_val , right_val ) => {","highlight_start":1,"highlight_end":30},{"text":"if ! ( * left_val == * right_val ) {","highlight_start":1,"highlight_end":37},{"text":"panic ! (","highlight_start":1,"highlight_end":10},{"text":"r#\"assertion failed: `(left == right)`","highlight_start":1,"highlight_end":39},{"text":"  left: `{:?}`,","highlight_start":1,"highlight_end":16},{"text":" right: `{:?}`: {}\"# ,","highlight_start":1,"highlight_end":23},{"text":"& * left_val , & * right_val , format_args ! ( $ ( $ arg ) + ) ) } } } } ) ;","highlight_start":1,"highlight_end":77}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"the trait `std::cmp::PartialEq<(&str,)>` is not implemented for `&str`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0277]: can't compare `&str` with `(&str,)`\n  --> /checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-expected-behavior.rs:59:5\n   |\nLL |     assert_eq!(\"Yeah\", dbg!(\"Yeah\", ));\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no implementation for `&str == (&str,)`\n   |\n   = help: the trait `std::cmp::PartialEq<(&str,)>` is not implemented for `&str`\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[01:07:12] {"message":"For more information about this error, try `rustc --explain E0277`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0277`.\n"}
[01:07:12] 
[01:07:12] ------------------------------------------
[01:07:12] 
---
[01:07:12] 
[01:07:12] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:516:22
[01:07:12] 
[01:07:12] 
[01:07:12] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:07:12] 
[01:07:12] 
[01:07:12] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:07:12] Build completed unsuccessfully in 0:04:20
[01:07:12] Build completed unsuccessfully in 0:04:20
[01:07:12] Makefile:48: recipe for target 'check' failed
[01:07:12] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02772f15
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Apr 10 05:09:17 UTC 2019
---
travis_time:end:03744de9:start=1554872959032350652,finish=1554872959039750671,duration=7400019
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0286d668
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01b5a916
$ dmesg | grep -i kill
