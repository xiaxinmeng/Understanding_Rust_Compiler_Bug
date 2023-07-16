\n"},"level":"error","spans":[{"file_name":"<::dollar_crate_external::external macros>","byte_start":149,"byte_end":176,"line_start":3,"line_end":4,"column_start":64,"column_end":17,"is_primary":true,"text":[{"text":"struct A ( $ crate :: S ) ; # [ derive ( dollar_crate :: d ) ] struct D (","highlight_start":64,"highlight_end":74},{"text":"$ crate :: S ) ; } ;","highlight_start":1,"highlight_end":17}],"label":"`D` redefined here","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/proc-macro/dollar-crate.rs","byte_start":767,"byte_end":802,"line_start":31,"line_end":31,"column_start":5,"column_end":40,"is_primary":false,"text":[{"text":"    dollar_crate_external::external!(); //~ ERROR the name `D` is defined multiple times","highlight_start":5,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"dollar_crate_external::external!","def_site_span":{"file_name":"<::dollar_crate_external::external macros>","byte_start":0,"byte_end":180,"line_start":1,"line_end":4,"column_start":1,"column_end":21,"is_primary":false,"text":[{"text":"(  ) => {","highlight_start":1,"highlight_end":10},{"text":"dollar_crate :: m ! { struct M ( $ crate :: S ) ; } # [ dollar_crate :: a ]","highlight_start":1,"highlight_end":76},{"text":"struct A ( $ crate :: S ) ; # [ derive ( dollar_crate :: d ) ] struct D (","highlight_start":1,"highlight_end":74},{"text":"$ crate :: S ) ; } ;","highlight_start":1,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"<::dollar_crate_external::external macros>","byte_start":149,"byte_end":176,"line_start":3,"line_end":4,"column_start":64,"column_end":17,"is_primary":true,"text":[{"text":"struct A ( $ crate :: S ) ; # [ derive ( dollar_crate :: d ) ] struct D (","highlight_start":64,"highlight_end":74},{"text":"$ crate :: S ) ; } ;","highlight_start":1,"highlight_end":17}],"label":"previous definition of the type `D` here","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/proc-macro/dollar-crate.rs","byte_start":767,"byte_end":802,"line_start":31,"line_end":31,"column_start":5,"column_end":40,"is_primary":false,"text":[{"text":"    dollar_crate_external::external!(); //~ ERROR the name `D` is defined multiple times","highlight_start":5,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"dollar_crate_external::external!","def_site_span":{"file_name":"<::dollar_crate_external::external macros>","byte_start":0,"byte_end":180,"line_start":1,"line_end":4,"column_start":1,"column_end":21,"is_primary":false,"text":[{"text":"(  ) => {","highlight_start":1,"highlight_end":10},{"text":"dollar_crate :: m ! { struct M ( $ crate :: S ) ; } # [ dollar_crate :: a ]","highlight_start":1,"highlight_end":76},{"text":"struct A ( $ crate :: S ) ; # [ derive ( dollar_crate :: d ) ] struct D (","highlight_start":1,"highlight_end":74},{"text":"$ crate :: S ) ; } ;","highlight_start":1,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`D` must be defined only once in the type namespace of this module","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0428]: the name `D` is defined multiple times\n  --> /checkout/src/test/ui/proc-macro/dollar-crate.rs:31:5\n   |\nLL |     dollar_crate_external::external!(); //~ ERROR the name `D` is defined multiple times\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |     |\n   |     `D` redefined here\n   |     previous definition of the type `D` here\n   |\n   = note: `D` must be defined only once in the type namespace of this module\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[00:44:25] thread 'main' panicked at 'src/librustc_resolve/macros.rs:878: inconsistent resolution for a macro', src/librustc/util/bug.rs:47:26
[00:44:25] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:44:25] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:44:25] {"message":"For more information about this error, try `rustc --explain E0428`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0428`.\n"}
[00:44:25] error: internal compiler error: unexpected panic
[00:44:25] 
[00:44:25] note: the compiler unexpectedly panicked. this is a bug.
[00:44:25] 
[00:44:25] 
[00:44:25] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:44:25] 
[00:44:25] note: rustc 1.33.0-nightly (995baf76c 2018-12-17) running on x86_64-unknown-linux-gnu
[00:44:25] 
[00:44:25] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C linker=cc
[00:44:25] 
[00:44:25] ------------------------------------------
[00:44:25] 
[00:44:25] thread '[ui] ui/proc-macro/dollar-crate.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3252:9
---
[00:44:25] test result: FAILED. 5152 passed; 1 failed; 28 ignored; 0 measured; 0 filtered out
[00:44:25] 
[00:44:25] 
[00:44:25] 
[00:44:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:44:25] 
[00:44:25] 
[00:44:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[00:44:25] Build completed unsuccessfully in 0:42:29
---
travis_time:end:00dfa458:start=1545011226132363080,finish=1545011226141337177,duration=8974097
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:11ca03ba
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0426f450
travis_time:start:0426f450
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:178d25aa
$ dmesg | grep -i kill
