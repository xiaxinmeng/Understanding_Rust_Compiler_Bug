plain
[00:48:20] ....................................................................................................
[00:48:23] ....................................................................................................
[00:48:28] ....................................................................................................
[00:48:32] ....................................................................................................
[00:48:38] ...........................................................F........................................
[00:48:49] ....i..............................................................................i................
[00:48:54] ....................................................................................................
[00:48:59] ....................................................................................................
[00:49:05] ....................................................................................................
[00:49:05] ....................................................................................................
[00:49:11] ................i.................iiiiiiiii...................................................
[00:49:11] 
[00:49:11] ---- [ui] ui/issue-50781.rs stdout ----
[00:49:11] diff of stderr:
[00:49:11] 
[00:49:11] 
[00:49:11] 10 LL | #![deny(where_clauses_object_safety)]
[00:49:11] 12    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:49:11] -    = note: for more information, see issue TBD
[00:49:11] -    = note: for more information, see issue TBD
[00:49:11] +    = note: for more information, see issue #5144e_start":16,"line_end":16,"column_start":5,"column_end":37,"is_primary":true,"text":[{"text":"    fn foo(&self) where Self: Trait; //~ ERROR the trait `X` cannot be made into an object","highlight_start":5,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issue-50781.rs","byte_start":475,"byte_end":502,"line_start":11,"line_end":11,"column_start":9,"column_end":36,"is_primary":true,"text":[{"text":"#![deny(where_clauses_object_safety)]","highlight_start":9,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"for more information, see issue #51443 <https://github.com/rust-lang/rust/issues/51443>","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"method `foo` references the `Self` type in where clauses","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: the trait `X` cannot be made into an object\n  --> /checkout/src/test/ui/issue-50781.rs:16:5\n   |\nLL |     fn foo(&self) where Self: Trait; //~ ERROR the trait `X` cannot be made into an object\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/issue-50781.rs:11:9\n   u/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:49:11] 
[00:49:11] 
[00:49:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:49:11] Build completed unsuccessfully in 0:02:40
[00:49:11] Build completed unsuccessfully in 0:02:40
[00:49:11] make: *** [check] Error 1
[00:49:11] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:278d9b28
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Jun  8 22:18:34 UTC 2018
./obj/build/bootstrap/debug/incremental/bootstrap-c730863262pt
103812 ./obj/build/bootstrap/debug/incremental/bootstrap-c730863262pt/s-f1t1g1aztd-ovzgu6-3m5keuwh0gmzh
102908 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu
102904 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
99524 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps
91280 ./obj/build/x86_64-unknown-linux-gnu/stage1
