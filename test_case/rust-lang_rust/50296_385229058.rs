plain
[00:41:58] ....................................................................................................
[00:42:04] ....................................................................................................
[00:42:09] ....................................................................................................
[00:42:16] ...................................i................................................................
[00:42:22] ...........i........................................F...............................................
[00:42:34] ....................................................................................................
[00:42:39] .........i....................................................................
[00:42:39] failures:
[00:42:39] 
[00:42:39] 
[00:42:39] ---- [ui] ui/raw_string_hash_count.rs stdout ----
[00:42:39]  normalized stderr:
[00:42:39] error: too many `#` symbols: raw strings may be delimited by up to 255 `#` symbols
[00:42:39]    |
[00:42:39]    |
[00:42:39] LL |     r################################################################################################################################################################################################################################################################"test"################################################################################################################################################################################################################################################################;
[00:42:39] 
[00:42:39] error: aborting due to previous error
[00:42:39] 
[00:42:39] 
[00:42:39] 
[00:42:39] 
[00:42:39] 
[00:42:39] The actual stderr differed from the expected stderr.
[00:42:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/raw_string_hash_count.stderr
[00:42:39] To update references, run this command from build directory:
[00:42:39] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'raw_string_hash_count.rs'
[00:42:39] error: 1 errors occurred comparing output.
[00:42:39] status: exit code: 101
[00:42:39] status: exit code: 101
[00:42:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/raw_string_hash_count.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/raw_string_hash_count.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/raw_string_hash_count.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:42:39] ------------------------------------------
[00:42:39] 
[00:42:39] ------------------------------------------
[00:42:39] stderr:
[00:42:39] stderr:
[00:42:39] ------------------------------------------
[00:42:39] {"message":"too many `#` symbols: raw strings may be delimited by up to 255 `#` symbols","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/raw_string_hash_count.rs","byte_start":510,"byte_end":767,"line_start":14,"line_end":14,"column_start":5,"column_end":262,"is_primary":true,"text":[{"text":"    r################################################################################################################################################################################################################################################################\"test\"################################################################################################################################################################################################################################################################;","highlight_start":5,"highlight_end":262}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error: too many `#` symbols: raw strings may be delimited by up to 255 `#` symbols\n  --> /checkout/src/test/ui/raw_string_hash_count.rs:14:5\n   |\nLL |     r################################################################################################################################################################################################################################################################\"test\"###t/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:42:39] 
[00:42:39] 
[00:42:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:42:39] Build completed unsuccessfully in 0:02:21
[00:42:39] Build completed unsuccessfully in 0:02:21
[00:42:39] Makefile:58: recipe for target 'check' failed
[00:42:39] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:11af5109
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
