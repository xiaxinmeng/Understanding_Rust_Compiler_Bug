plain
[00:16:33] 339 |                 let args_to_check;
[00:16:33]     |                     ^^^^^^^^^^^^^
[00:16:33]     |                     |
[00:16:33]     |                     cannot infer type
[00:16:33]     |                     consider giving `args_to_check` a type
[00:16:33] error: aborting due to previous error
[00:16:33] 
[00:16:33] For more information about this error, try `rustc --explain E0282`.
[00:16:33] error: Could not compile `rustc_lint`.
---
travis_time:end:098aae79:start=1538501030341940661,finish=1538501030346440223,duration=4499562
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:27ef9872
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0f54cea4
travis_time:start:0f54cea4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:011a87b9
$ dmesg | grep -i kill
