\n\nWith this approach, x and y share ownership of the data via the `Rc` (reference\ncount type). `RefCell` essentially performs runtime borrow checking: ensuring\nthat at most one writer or multiple readers can access the data at any one time.\n\nIf you wish to learn more about ownership in Rust, start with the chapter in the\nBook:\n\nhttps://doc.rust-lang.org/book/first-edition/ownership.html\n"},"level":"error","spans":[{"file_name":"<dbg macros>","byte_start":36,"byte_end":39,"line_start":3,"line_end":3,"column_start":1,"column_end":4,"is_primary":false,"text":[{"text":"tmp => {","highlight_start":1,"highlight_end":4}],"label":"value moved here","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-move-semantics.rs","byte_start":184,"byte_end":191,"line_start":10,"line_end":10,"column_start":13,"column_end":20,"is_primary":false,"text":[{"text":"    let _ = dbg!(a);","highlight_start":13,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"dbg!","def_site_span":{"file_name":"<dbg macros>","byte_start":0,"byte_end":152,"line_start":1,"line_end":6,"column_start":1,"column_end":18,"is_primary":false,"text":[{"text":"( $ val : expr ) => {","highlight_start":1,"highlight_end":22},{"text":"match $ val {","highlight_start":1,"highlight_end":14},{"text":"tmp => {","highlight_start":1,"highlight_end":9},{"text":"eprintln ! (","highlight_start":1,"highlight_end":13},{"text":"\"[{}:{}] {} = {:#?}\" , file ! (  ) , line ! (  ) , stringify ! ( $ val ) , &","highlight_start":1,"highlight_end":77},{"text":"tmp ) ; tmp } } }","highlight_start":1,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"/checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-move-semantics.rs","byte_start":210,"byte_end":211,"line_start":11,"line_end":11,"column_start":18,"column_end":19,"is_primary":true,"text":[{"text":"    let _ = dbg!(a);","highlight_start":18,"highlight_end":19}],"label":"value used here after move","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"move occurs because `a` has type `NoCopy`, which does not implement the `Copy` trait","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0382]: use of moved value: `a`\n  --> /checkout/src/test/ui/rfc-2361-dbg-macro/dbg-macro-move-semantics.rs:11:18\n   |\nLL |     let _ = dbg!(a);\n   |             ------- value moved here\nLL |     let _ = dbg!(a);\n   |                  ^ value used here after move\n   |\n   = note: move occurs because `a` has type `NoCopy`, which does not implement the `Copy` trait\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[01:03:52] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:03:52] {"message":"For more information about this error, try `rustc --explain E0382`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0382`.\n"}
[01:03:52] ------------------------------------------
[01:03:52] 
[01:03:52] 
[01:03:52] thread '[ui (nll)] ui/rfc-2361-dbg-macro/dbg-macro-move-semantics.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[01:03:52] 
[01:03:52] 
[01:03:52] failures:
[01:03:52] failures:
[01:03:52]     [ui (nll)] ui/rfc-2361-dbg-macro/dbg-macro-move-semantics.rs
[01:03:52] test result: FAILED. 6720 passed; 1 failed; 88 ignored; 0 measured; 0 filtered out
[01:03:52] 
[01:03:52] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:496:22
[01:03:52] 
[01:03:52] 
[01:03:52] 
[01:03:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[01:03:52] 
[01:03:52] 
[01:03:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:03:52] Build completed unsuccessfully in 0:15:02
[01:03:52] Build completed unsuccessfully in 0:15:02
[01:03:52] Makefile:58: recipe for target 'check' failed
[01:03:52] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b5b2f56
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:04fa5074:start=1537454605543258552,finish=1537454605547278048,duration=4019496
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0395b4be
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ec5ff88
travis_time:start:0ec5ff88
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0e6ee1cb
$ dmesg | grep -i kill
