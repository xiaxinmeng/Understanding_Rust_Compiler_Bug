plain
[00:19:54]    Compiling cc v1.0.25
[00:19:54]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:19:54]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:19:54]    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
[00:19:54] error: internal compiler error: pushing duplicate keyword to expected_tokens
[00:19:54]     |
[00:19:54]     |
[00:19:54] 137 | mod macros;
[00:19:54] 
[00:19:54] error: aborting due to previous error
[00:19:54] 
[00:19:54] error: Could not compile `core`.
---
travis_time:end:0a8ecd48:start=1538708157799517547,finish=1538708157803980057,duration=4462510
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:017c3980
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2773c5f6
travis_time:start:2773c5f6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: 
