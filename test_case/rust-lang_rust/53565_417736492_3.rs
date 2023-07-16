\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issue-53565.rs","byte_start":482,"byte_end":485,"line_start":10,"line_end":10,"column_start":17,"column_end":20,"is_primary":true,"text":[{"text":"use std::time::{foo, bar, buzz};","highlight_start":17,"highlight_end":20}],"label":"no `foo` in `time`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issue-53565.rs","byte_start":487,"byte_end":490,"line_start":10,"line_end":10,"column_start":22,"column_end":25,"is_primary":true,"text":[{"text":"use std::time::{foo, bar, buzz};","highlight_start":22,"highlight_end":25}],"label":"no `bar` in `time`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/issue-53565.rs","byte_start":492,"byte_end":496,"line_start":10,"line_end":10,"column_start":27,"column_end":31,"is_primary":true,"text":[{"text":"use std::tessfully in 0:03:03
[00:45:46] make: *** [check] Error 1
[00:45:46] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:23adbb39
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
151200 ./src/tools/clang
149120 ./src/llvm-emscripten/test
148964 ./obj/build/bootstrap/debug/incremental
134532 ./obj/build/bootstrap/debug/incremental/bootstrap-11nz4fw202v9g
134528 ./obj/build/bootstrap/debug/incremental/bootstrap-11nz4fw202v9g/s-f4dehdm17m-1ighvpz-3aquzm2prkry8
129812 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
1or CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:009c58e8
travis_time:start:009c58e8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:10ff9758
$ dmesg | grep -i kill
