plain
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:50:35] 
[00:50:35] running 4571 tests
[00:50:38] ......................F............................................................................. 100/4571
[00:50:45] .................................................................................................... 300/4571
[00:50:48] .................................................................................................... 400/4571
[00:50:51] .................................................................................................... 500/4571
[00:50:54] .......................i............................................................................ 600/4571
[00:50:54] .......................i............................................................................ 600/4571
[00:50:59] .................................................................................................... 700/4571
[00:51:04] ................................i...........i....................................................... 800/4571
[00:51:08] ...................................................iiiii............................................ 900/4571
[00:51:11] .................................................................................................... 1000/4571
[00:51:13] .................................................................................................... 1100/4571
[00:51:15] ...........................................................................................F........ 1200/4571
[00:51:20] .................................................................................................... 1400/4571
[00:51:23] .........................................i.......................................................... 1500/4571
[00:51:27] ........i........................................................................................... 1600/4571
[00:51:30] .................................................................................................... 1700/4571
---
[00:52:56] .................................................................................................... 4100/4571
[00:53:00] .......................................................i............................................ 4200/4571
[00:53:03] .................................................................................................... 4300/4571
[00:53:07] .................................................................................................... 4400/4571
ld/x86_64-unknown-linux-gnu/test/ui/error-codes/E0637/E0637.stderr
[00:53:11] To update references, rerun the tests and pass the `--bless` flag
[00:53:11] To only update this specific test, also pass `--test-args error-codes/E0637.rs`
[00:53:11] error: 1 errors occurred comparing output.
[00:53:11] status: exit code: 1
[00:53:11] status: exit code: 1
[00:53:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0637.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0637/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0637/auxiliary" "-A" "unused"
[00:53:11] ------------------------------------------
[00:53:11] 
[00:53:11] ------------------------------------------
[00:53:11] stderr:
[00:53:11] stderr:
[00:53:11] ------------------------------------------
[00:53:11] {"message":"invalid lifetime bound name: `'_`","code":{"code":"E0637","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0637.rs","byte_start":482,"byte_end":484,"line_start":11,"line_end":11,"column_start":16,"column_end":18,"is_primary":true,"text":[{"text":"struct Foo<'a: '_>(&'a u8); //~ ERROR invalid lifetime bound name: `'_`","highlight_start":16,"highlight_end":18}],"label":"`'_` is a reserved lifetime name","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"reE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c31a807
travis_time:start:0c31a807
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0daa2d74
$ dmesg | grep -i kill
