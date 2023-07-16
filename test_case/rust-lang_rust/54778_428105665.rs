plain
[00:44:54] ................................i...........i....................................................... 800/4571
[00:44:57] ...................................................iiiii............................................ 900/4571
[00:45:00] .................................................................................................... 1000/4571
[00:45:02] .................................................................................................... 1100/4571
[00:45:04] ...........................................................................................F........ 1200/4571
[00:45:09] .................................................................................................... 1400/4571
[00:45:12] .........................................i.......................................................... 1500/4571
[00:45:15] ........i........................................................................................... 1600/4571
[00:45:18] .................................................................................................... 1700/4571
---
[00:46:54] 22 
[00:46:54] 
[00:46:54] 
[00:46:54] The actual stderr differed from the expected stderr.
[00:46:54] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0637/E0637.stderr
[00:46:54] To update references, rerun the tests and pass the `--bless` flag
[00:46:54] To only update this specific test, also pass `--test-args error-codes/E0637.rs`
[00:46:54] error: 1 errors occurred comparing output.
[00:46:54] status: exit code: 1
[00:46:54] status: exit code: 1
[00:46:54] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0637.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0637/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0637/auxiliary" "-A" "unused"
[00:46:54] ------------------------------------------
[00:46:54] 
[00:46:54] ------------------------------------------
[00:46:54] stderr:
[00:46:54] stderr:
[00:46:54] ------------------------------------------
[00:46:54] {"message":"invalid lifetime bound name: `'_`","code":{"code":"E0637","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0637.rs","byte_start":482,"byte_end":484,"line_start":11,"line_end":11,"column_start":16,"column_end":18,"is_primary":true,"text":[{"text":"struct Foo<'a: '_>(&'a u8); //~ ERROR invalid lifetime bound name: `'_`","highli/stage1-rustc/x86_64-unknown-linux-gnu
132600 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
130808 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu
130804 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
123768 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
---
75904 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot
75900 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib
75896 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib
75892 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu
75888 ./obj/build/x86_64-unknownkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:153a0a77
travis_time:start:153a0a77
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:020237dc
$ dmesg | grep -i kill
