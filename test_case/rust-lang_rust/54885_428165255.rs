plain
[00:46:18] .................................................................................................... 2500/4572
[00:46:22] .................................................................................................... 2600/4572
[00:46:25] .................................................................................................... 2700/4572
[00:46:28] .................................................................................................... 2800/4572
[00:46:32] ..................................................................................F................. 2900/4572
[00:46:38] ................................................................................................i.i. 3100/4572
[00:46:41] .ii................................................................................................. 3200/4572
[00:46:44] .................................................................................................... 3300/4572
[00:46:47] ..........................................i......................................................... 3400/4572
---
[00:47:24] 
[00:47:24] ---- [ui] ui/lint/unused_parens_json_suggestion.rs stdout ----
[00:47:24] diff of stderr:
[00:47:24] 
[00:47:24] 8   "spans": [
[00:47:24] 9     {
[00:47:24] 10       "file_name": "$DIR/unused_parens_json_suggestion.rs",
[00:47:24] -       "byte_start": 1043,
[00:47:24] -       "byte_end": 1056,
[00:47:24] -       "line_start": 25,
[00:47:24] -       "line_end": 25,
[00:47:24] +       "byte_start": 1071,
[00:47:24] +       "byte_end": 1084,
[00:47:24] +       "line_start": 26,
[00:47:24] +       "line_end": 26,
[00:47:24] 15       "column_start": 14,
[00:47:24] 16       "column_end": 27,
[00:47:24] 17       "is_primary": true,
[00:47:24] 
[00:47:24] 66       "spans": [
[00:47:24] 67         {
[00:47:24] 68           "file_name": "$DIR/unused_parens_json_suggestion.rs",
[00:47:24] -           "byte_start": 1043,
[00:47:24] -           "byte_end": 1056,
[00:47:24] -           "line_start": 25,
[00:47:24] -           "line_end": 25,
[00:47:24] +           "byte_start": 1071,
[00:47:24] +           "byte_end": 1084,
[00:47:24] +           "line_start": 26,
[00:47:24] +           "line_end": 26,
[00:47:24] 73           "column_start": 14,
[00:47:24] 74           "column_end": 27,
[00:47:24] 75           "is_primary": true,
[00:47:24] 91     }
[00:47:24] 92   ],
[00:47:24] 92   ],
[00:47:24] 93   "rendered": "warning: unnecessary parentheses around assigned value
[00:47:24] -   --> $DIR/unused_parens_json_suggestion.rs:25:14
[00:47:24] +   --> $DIR/unused_parens_json_suggestion.rs:26:14
[00:47:24] 95    |
[00:47:24] 96 LL |     let _a = (1 / (2 + 3));
[00:47:24] 
[00:47:24] 
[00:47:24] The actual stderr differed from the expected stderr.
[00:47:24] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_json_suggestion/unused_parens_json_suggestion.stderr
[00:47:24] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_json_suggestion/unused_parens_json_suggestion.stderr
[00:47:24] To update references, rerun the tests and pass the `--bless` flag
[00:47:24] To only update this specific test, also pass `--test-args lint/unused_parens_json_suggestion.rs`
[00:47:24] error: 1 errors occurred comparing output.
[00:47:24] status: exit code: 0
[00:47:24] status: exit code: 0
[00:47:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused_parens_json_suggestion.rs" "--target=x86_6ability": null,
[00:47:24]       "expansion": null
[00:47:24]   ],
[00:47:24]   ],
[00:47:24]   "children": [
[00:47:24]     {
[00:47:24]       "message": "lint level defined here",
[00:47:24]       "code": null,
[00:47:24]       "level": "note",
[00:47:24]       "spans": [
[00:47:24]         {
[00:47:24]           "file_name": "/checkout/src/test/ui/lint/unused_parens_json_suggestion.rs",
[00:47:24]           "byte_start": 889,
[00:47:24]           "byte_end": 902,
[00:47:24]           "line_start": 20,
[00:47:24]           "line_end": 20,
[00:47:24]           "column_start": 9,
[00:47:24]           "column_end": 22,
[00:47:24]           "is_primary": true,
[00:47:24]           "text": [
[00:47:24]             {
[00:47:24]               "text": "#![warn(unused_parens)]",
[00:47:24]               "highlight_start": 9,
[00:47:24]               "highlight_end": 22
[00:47:24]           ],
[00:47:24]           ],
[00:47:24]           "label": null,
[00:47:24]           "suggested_replacement": null,
[00:47:24]           "suggestion_applicability": null,
[00:47:24]           "expansion": null
[00:47:24]       ],
[00:47:24]       ],
[00:47:24]       "children": [],
[00:47:24]       "rendered": null
[00:47:24]     {
[00:47:24]     {
[00:47:24]       "message": "remove these parentheses",
[00:47:24]       "code": null,
[00:47:24]       "level": "help",
[00:47:24]       "spans": [
[00:47:24]         {
[00:47:24]           "file_name": "/checkout/src/test/ui/lint/unused_parens_json_suggestion.rs",
[00:47:24]           "byte_start": 1071,
[00:47:24]           "byte_end": 1084,
[00:47:24]  .rs
[00:47:24] test result: FAILED. 4551 passed; 1 failed; 20 ignored; 0 measured; 0 filtered out
[00:47:24] 
[00:47:24] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:499:22
[00:47:24] 
[00:47:24] 
[00:47:24] 
[00:47:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "5.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[00:47:24] 
[00:47:24] 
[00:47:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:47:24] Build completed unsuccessfully in 0:03:23
[00:47:24] Build completed unsuccessfully in 0:03:23
[00:47:24] Makefile:58: recipe for target 'check' failed
[00:47:24] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a2afa44
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:258abd2c:start=1539086481341711806,finish=1539086481346082347,duration=4370541
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0d0d7a0b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:20dc4b58
travis_time:start:20dc4b58
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1c874a6a
$ dmesg | grep -i kill
