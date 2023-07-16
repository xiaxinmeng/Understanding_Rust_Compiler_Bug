\n"},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/regions/regions-ref-in-fn-arg.rs","byte_start":297,"byte_end":298,"line_start":11,"line_end":11,"column_start":22,"column_end":23,"is_primary":true,"text":[{"text":"    with(|box ref x| x) //~ ERROR borrowed value does not live long enough","highlight_start":22,"highlight_end":23}],"label":"returns a value referencing data owned by the current function","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/regions/regions-ref-in-fn-arg.rs","byte_start":286,"byte_end":295,"line_start":11,"line_end":11,"column_start":11,"column_end":20,"is_primary":false,"text":[{"text":"    with(|box ref x| x) //~ ERROR borrowed value does not live long enough","highlight_start":11,"highlight_end":20}],"label":"function parameter borrowed here","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this error has been downgraded to a warning for backwards compatibility with previous releases","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"this represents potential undefined behavior in your code and this warning will become a hard error in the future","code":null,"level":"warning","spans":[],"children":[],"rendered":null}],"rendered":"warning[E0515]: cannot return value referencing function parameter\n  --> /checkout/src/test/ui/regions/regions-ref-in-fn-arg.rs:11:22\n   |\nLL |     with(|box ref x| x) //~ ERROR borrowed value does not live long enough\n   |           ---------  ^ returns a value referencing data owned by the current function\n   |           |\n   |           function parameter borrowed here\n   |\n   = warning: this error has been downgraded to a warning for backwards compatibility with previous releases\n   = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future\n\n"}
[01:38:31] {"message":"For more information about this error, try `rustc --explain E0515`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0515`.\n"}
[01:38:31] 
[01:38:31] ------------------------------------------
[01:38:31] 
---
[01:38:31] test result: FAILED. 5354 passed; 7 failed; 90 ignored; 0 measured; 0 filtered out
[01:38:31] 
[01:38:31] 
[01:38:31] 
[01:38:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[01:38:31] 
[01:38:31] 
[01:38:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:38:31] Build completed unsuccessfully in 0:08:02
---
travis_time:end:1a7535b0:start=1552233424157945139,finish=1552233424166115027,duration=8169888
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0338f548
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06f73617
travis_time:start:06f73617
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00dd05ec
$ dmesg | grep -i kill
