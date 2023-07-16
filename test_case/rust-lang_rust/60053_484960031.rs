plain
[01:23:25]     |
[01:23:25] 353 | extern crate serde;
[01:23:25]     | ^^^^^^^^^^^^^^^^^^^
[01:23:25]     |
[01:23:25]     = note: candidates:
[01:23:25]             crate `serde_derive`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-8733351e2effc45d.so
[01:23:25]             crate `serde_derive`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libserde_derive-a2807fbab116827f.so
[01:23:25] 
[01:23:25] error[E0461]: couldn't find crate `serde_derive` with expected target triple x86_64-unknown-linux-gnu which `serde` depends on
[01:23:25]     |
[01:23:25] 353 | extern crate serde;
[01:23:25]     | ^^^^^^^^^^^^^^^^^^^
[01:23:25]     |
[01:23:25]     |
[01:23:25]     = note: the following crate versions were found:
[01:23:25]             crate `serde_derive`, target triple x86_64-unknown-freebsd: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-freebsd/release/deps/libserde_derive-e36265ac4b6d708b.so
[01:23:25] error: aborting due to 2 previous errors
[01:23:25] 
[01:23:26] [RUSTC-TIMING] serde_json test:false 0.231
[01:23:26] error: Could not compile `serde_json`.
---
travis_time:end:082793ee:start=1555693877057959826,finish=1555693877065429857,duration=7470031
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2275b9e0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05b0b73e
travis_time:start:05b0b73e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:3305ef4b
$ dmesg | grep -i kill
