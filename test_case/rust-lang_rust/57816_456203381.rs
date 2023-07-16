plain
[00:05:29] [RUSTC-TIMING] syntax_pos test:false 3.910
[00:05:29]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:05:40] [RUSTC-TIMING] rustc_target test:false 15.161
[00:05:41] [RUSTC-TIMING] rustc_errors test:false 12.043
[00:05:42] error[E0412]: cannot find type `ParenthesisedArgs` in this scope
[00:05:42]    --> src/libsyntax/ast.rs:195:6
[00:05:42]     |
[00:05:42] 195 | impl ParenthesisedArgs {
[00:05:42]     |      ^^^^^^^^^^^^^^^^^ did you mean `ParenthesizedArgs`?
[00:05:47] error: aborting due to previous error
[00:05:47] 
[00:05:47] For more information about this error, try `rustc --explain E0412`.
[00:05:47] [RUSTC-TIMING] syntax test:false 6.190
---
travis_time:end:050a4e5c:start=1548106080640405879,finish=1548106080655733232,duration=15327353
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08c353ef
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03b97ac0
travis_time:start:03b97ac0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:040be123
$ dmesg | grep -i kill
