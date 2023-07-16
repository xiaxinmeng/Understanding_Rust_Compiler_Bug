plain
[00:37:29]    Compiling aho-corasick v0.6.6
[00:37:34]    Compiling tempfile v3.0.2
[00:37:54]    Compiling minifier v0.0.14
[00:37:57]    Compiling rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[00:38:05] error: the feature `entry_and_modify` has been stable since 1.26.0 and no longer requires an attribute to enable
[00:38:05]    |
[00:38:05]    |
[00:38:05] 25 | #![feature(entry_and_modify)]
[00:38:05]    |
[00:38:05]    = note: `-D stable-features` implied by `-D warnings`
[00:38:05] 
[00:38:05] error: aborting due to previous error
---
travis_time:end:009f0c84:start=1532361941744774500,finish=1532361941752022532,duration=7248032
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1284a230
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:22bd20ef
travis_time:start:22bd20ef
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0720201e
$ dmesg | grep -i kill
