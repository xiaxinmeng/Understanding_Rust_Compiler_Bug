plain
[01:23:52]     |
[01:23:52] 273 |                     new_capacity = last_chunk.storage.cap();
[01:23:52]     |                                                       ^^^ private field, not a method
[01:23:52] 
[01:23:52] error[E0599]: no method named `cap` found for type `alloc::raw_vec::RawVec<u8>` in the current scope
[01:23:52]     |
[01:23:52] 408 |                     new_capacity = last_chunk.storage.cap();
[01:23:52]     |                                                       ^^^ private field, not a method
[01:23:52] 
---
[01:24:34]     |
[01:24:34] 273 |                     new_capacity = last_chunk.storage.cap();
[01:24:34]     |                                                       ^^^ private field, not a method
[01:24:34] 
[01:24:34] error[E0599]: no method named `cap` found for type `alloc::raw_vec::RawVec<u8>` in the current scope
[01:24:34]     |
[01:24:34] 408 |                     new_capacity = last_chunk.storage.cap();
[01:24:34]     |                                                       ^^^ private field, not a method
[01:24:34] 
---
travis_time:end:2f481057:start=1561543232659905909,finish=1561543232667017764,duration=7111855
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04c852f8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05198e40
travis_time:start:05198e40
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1d4099f6
$ dmesg | grep -i kill
