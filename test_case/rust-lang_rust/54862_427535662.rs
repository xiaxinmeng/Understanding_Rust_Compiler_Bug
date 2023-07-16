plain
[00:49:42] .................................................................................................... 1000/4560
[00:49:44] .................................................................................................... 1100/4560
[00:49:46] .................................................................................................... 1200/4560
[00:49:49] .................................................................................................... 1300/4560
[00:49:51] ..............FF.................................................................................... 1400/4560
[00:49:57] .................i.................................................................................. 1600/4560
[00:50:00] .................................................................................................... 1700/4560
[00:50:04] .................................................................................................... 1800/4560
[00:50:07] ..............................................i..................................................... 1900/4560
---
[00:51:39] ---- [ui] ui/feature-gates/feature-gate-cfg-attr-multi-2.rs stdout ----
[00:51:39] 
[00:51:39] error: ui test compiled successfully!
[00:51:39] status: exit code: 0
[00:51:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-cfg-attr-multi-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-cfg-attr-multi-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-cfg-attr-multi-2/auxiliary" "-A" "unused"
[00:51:39] ------------------------------------------
[00:51:39] 
[00:51:39] ------------------------------------------
[00:51:39] stderr:
[00:51:39] stderr:
[00:51:39] ------------------------------------------
[00:51:39] 
[00:51:39] ------------------------------------------
[00:51:39] 
[00:51:39] thread '[ui] ui/feature-gates/feature-gate-cfg-attr-multi-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
[00:kout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1ff3b324
travis_time:start:1ff3b324
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0103086c
$ dmesg | grep -i kill
