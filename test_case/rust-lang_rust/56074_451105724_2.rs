\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/impl-trait/infinite-impl-trait-issue-38064.rs","byte_start":293,"byte_end":302,"line_start":14,"line_end":14,"column_start":13,"column_end":22,"is_primary":true,"text":[{"text":"fn bar() -> impl Quux { //~ opaque type expands to a recursive type","highlight_start":13,"highlight_end":22}],"label":"expands to self-referential type","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/impl-trait/infinite-impl-trait-issue-38064.rs","byte_start":293,"byte_end":302,"line_start":14,"line_end":14,"column_start":13,"column_end":22,"is_primary":false,"text":[{"text":"fn bar() -> impl Quux { //~ opaque type expands to a recursive type","highlight_start":13,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"desugaring of `existential type`","def_site_span":{"file_name":"/checkout/src/test/ui/impl-trait/infinite-impl-trait-issue-38064.rs","byte_start":293,"byte_end":302,"line_start":14,"line_end":14,"column_start":13,"column_end":22,"is_primary":false,"text":[{"text":"fn bar() -> impl Quux { //~ opaque type expands to a recursive type","highlight_start":13,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"expanded type is `bar::Bar<foo::Foo<impl Quux>>`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0720]: opaque type expands to a recursive type\n  --> /checkout/src/test/ui/impl-trait/infinite-impl-trait-issue-38064.rs:14:13\n   |\nLL | fn bar() -> impl Quux { //~ opaque type expands to a recursive type\n   |             ^^^^^^^^^ expands to self-referential type\n   |\n   = note: expanded type is `bar::Bar<foo::Foo<impl Quux>>`\n\n"}
[00:48:39] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:48:39] {"message":"For more information about this error, try `rustc --explain E0720`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0720`.\n"}
[00:48:39] ------------------------------------------
[00:48:39] 
[00:48:39] thread '[ui] ui/impl-trait/infinite-impl-trait-issue-38064.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[00:48:39] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:48:39] 
[00:48:39] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[00:48:39] 
[00:48:39] 
[00:48:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0svn\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:48:39] 
[00:48:39] 
[00:48:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[00:48:39] Build completed unsuccessfully in 0:46:24
---
travis_time:end:371b0910:start=1546511344068181903,finish=1546511344074849411,duration=6667508
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f702870
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d5db000
travis_time:start:0d5db000
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a1dd79a
$ dmesg | grep -i kill
