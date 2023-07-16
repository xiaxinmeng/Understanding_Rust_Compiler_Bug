plain
[01:03:34] 
[01:03:34] ---- [ui] ui/specialization/specialization-overlap-hygiene.rs stdout ----
[01:03:34] diff of stderr:
[01:03:34] 
[01:03:34] 4 LL |     fn f() {}
[01:03:34] 5    |     --------- other definition for `f`
[01:03:34] 6 ...
[01:03:34] - LL |    fn f() {} //~ ERROR duplicate definitions with name `f`
[01:03:34] + LL |    fn f() {}
[01:03:34] 8    |    ^^^^^^^^^ duplicate definitions for `f`
[01:03:34] 10 error: aborting due to previous error
[01:03:34] 
[01:03:34] 
[01:03:34] The actual stderr differed from the expected stderr.
[01:03:34] The actual stderr differed from the expected stderr.
[01:03:34] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/specialization-overlap-hygiene/specialization-overlap-hygiene.stderr
[01:03:34] To update references, rerun the tests and pass the `--bless` flag
[01:03:34] To only update this specific test, also pass `--test-args specialization/specialization-overlap-hygiene.rs`
[01:03:34] error: 1 errors occurred comparing output.
[01:03:34] status: exit code: 1
[01:03:34] status: exit code: 1
[01:03:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/specialization-overlap-hygiene.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/specialization-overlap-hygiene/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/specialization-overlap-hygiene/auxiliary" "-A" "unused"
[01:03:34] ------------------------------------------
[01:03:34] 
[01:03:34] ------------------------------------------
[01:03:34] stderr:
[01:03:34] stderr:
[01:03:34] ------------------------------------------
[01:03:34] {"message":"duplicate definitions with name `f`","code":{"code":"E0592","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/specialization/specialization-overlap-hygiene.rs","byte_start":147,"byte_end":156,"line_start":13,"line_end":13,"column_start":4,"column_end":13,"is_primary":true,"text":[{"text":"   fn f() {} //~ ERROR duplicate definitions with name `f`","highlight_start":4,"highlight_end":13}],"label":"duplicate definitions for `f`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/specialization/specialization-overlap-hygiene.rs","byte_start":79,"byte_end":88,"line_start":6,"line_end":6,"column_start":5,"column_end":14,"is_primary":false,"text":[{"text":"    fn f() {}","highlight_start":5,"highlight_end":14}],"label":"other definition for `f`","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/specialization/specialization-overlap-hygiene.rs","byte_start":237,"byte_end":256,"line_start":17,"line_end":17,"column_start":5,"column_end":24,"is_primary":false,"text":[{"text":"    define_f_legacy!();","highlight_start":5,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"define_f_legacy!","def_site_span":{"file_name":"/checkout/src/test/ui/specialization/specialization-overlap-hygiene.rs","byte_start":36,"byte_end":91,"line_start":5,"line_end":7,"column_start":1,"column_end":3,"is_primary":false,"text":[{"text":"macro_rules! define_f_legacy { () => {","highlight_start":1,"highlight_end":39},{"text":"    fn f() {}","highlight_start":1,"highlight_end":14},{"text":"}}","highlight_start":1,"highlight_end":3}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error[E0592]: duplicate definitions with name `f`\n  --> /checkout/src/test/ui/specialization/specialization-overlap-hygiene.rs:13:4\n   |\nLL |     fn f() {}\n   |     --------- other definition for `f`\n...\nLL |    fn f() {} //~ ERROR duplicate definitions with name `f`\n   |    ^^^^^^^^^ duplicate definitions for `f`\n\n"}
[01:03:34] {"message":"For more information about this error, try `rustc --explain E0592`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0592`.\n"}
[01:03:34] 
[01:03:34] ------------------------------------------
[01:03:34] 
---
[01:03:34] 
[01:03:34] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:496:22
[01:03:34] 
[01:03:34] 
[01:03:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0-rust-1.35.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:03:34] 
[01:03:34] 
[01:03:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
[01:03:34] Build completed unsuccessfully in 1:00:34
---
travis_time:end:061300b8:start=1553194197393910888,finish=1553194197413231549,duration=19320661
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0e139b61
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00a231d6
travis_time:start:00a231d6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:143483cd
$ dmesg | grep -i kill
