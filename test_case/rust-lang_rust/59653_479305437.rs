plain
[01:31:50] 
[01:31:51] error: unused import: `ErrorKind`
[01:31:51]  --> src/libstd/sys/redox/fs.rs:5:30
[01:31:51]   |
[01:31:51] 5 | use crate::io::{self, Error, ErrorKind, SeekFrom};
[01:31:51]   |
[01:31:51]   = note: `-D unused-imports` implied by `-D warnings`
[01:31:51] 
[01:31:53] error: aborting due to previous error
---
travis_time:end:081afbe1:start=1554258581042247853,finish=1554258581061669671,duration=19421818
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b22b871
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ea379e0
travis_time:start:0ea379e0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2b3db2da
$ dmesg | grep -i kill
