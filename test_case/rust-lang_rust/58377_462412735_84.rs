\n\nThis will fail because the compiler does not know which instance of `Foo` to\ncall `bar` on. Change `Foo::bar()` to `Foo::<T>::bar()` to resolve the error.\n"},"level":"error","spans":[{"file_name":"<::alloc::macros::vec macros>","byte_start":127,"byte_end":144,"line_start":3,"line_end":3,"column_start":13,"column_end":30,"is_primary":true,"text":[{"text":"{ let tmp = [ $ ( $ x ) , * ] ; < [ _ ] > :: into_vec ( box tmp ) } ) ; (","highlight_start":13,"highlight_end":30}],"label":"cannot infer type","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/type/type-check/cannot_infer_local_or_vec_in_tuples.rs","byte_start":29,"byte_end":35,"line_start":2,"line_end":2,"column_start":18,"column_end":24,"is_primary":false,"text":[{"text":"    let (x, ) = (vec![], );","highlight_start":18,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"vec!","def_site_span":{"file_name":"<::alloc::macros::vec macros>","byte_start":0,"byte_end":242,"line_start":1,"line_end":4,"column_start":1,"column_end":54,"is_primary":false,"text":[{"text":"( $ elem : expr ; $ n : expr ) => (","highlight_start":1,"highlight_end":36},{"text":"$ crate :: vec :: from_elem ( $ elem , $ n ) ) ; ( $ ( $ x : expr ) , * ) => (","highlight_start":1,"highlight_end":79},{"text":"{ let tmp = [ $ ( $ x ) , * ] ; < [ _ ] > :: into_vec ( box tmp ) } ) ; (","highlight_start":1,"highlight_end":74},{"text":"$ ( $ x : expr , ) * ) => ( vec ! [ $ ( $ x ) , * ] )","highlight_start":1,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"/checkout/src/test/ui/type/type-check/cannot_infer_local_or_vec_in_tuples.rs","byte_start":20,"byte_end":25,"line_start":2,"line_end":2,"column_start":9,"column_end":14,"is_primary":false,"text":[{"text":"    let (x, ) = (vec![], );","highlight_start":9,"highlight_end":14}],"label":"consider giving the pattern a type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0282]: type annotations needed\n  --> /checkout/src/test/ui/type/type-check/cannot_infer_local_or_vec_in_tuples.rs:2:18\n   |\nLL |     let (x, ) = (vec![], );\n   |         -----    ^^^^^^ cannot infer type\n   |         |\n   |         consider giving the pattern a type\n   |\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[01:05:24] {"message":"For more information about this error, try `rustc --explain E0282`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0282`.\n"}
[01:05:24] 
[01:05:24] ------------------------------------------
[01:05:24] 
---
[01:05:24] 
[01:05:24] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:05:24] 
[01:05:24] 
[01:05:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:05:24] 
[01:05:24] 
[01:05:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:05:24] Build completed unsuccessfully in 0:04:13
[01:05:24] Build completed unsuccessfully in 0:04:13
[01:05:24] Makefile:48: recipe for target 'check' failed
[01:05:24] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2e9a7ae5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Feb 11 17:16:30 UTC 2019
---
travis_time:end:14b08693:start=1549905392090445099,finish=1549905392098303916,duration=7858817
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:086f3dc3
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00a822f8
$ dmesg | grep -i kill
