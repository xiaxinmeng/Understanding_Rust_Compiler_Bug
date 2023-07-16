plain
[00:01:42] configure: llvm.ccache          := sccache
[00:01:42] configure: build.openssl-static := True
[00:01:42] configure: build.build          := x86_64-unknown-linux-gnu
[00:01:42] configure: target.x86_64-unknown-linux-gnu.llvm-config := /usr/lib/llvm-5.0/bin/l ...
[00:01:42] configure: build.missing-tools  := True
[00:01:42] Traceback (most recent call last):
[00:01:42] Traceback (most recent call last):
[00:01:42]   File "/checkout/src/bootstrap/configure.py", line 437, in <module>
[00:01:42]     configure_section(sections[section_key], section_config)
[00:01:42]   File "/checkout/src/bootstrap/configure.py", line 425, in configure_section
[00:01:42]     raise RuntimeError("failed to find config line for {}".format(key))
[00:01:42] RuntimeError: failed to find config line for missing-tools

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:0ed48420
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:05c6b077:start=1538329281611597769,finish=1538329281616189965,duration=4592196
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00b3018d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b3c3bb5
travis_time:start:0b3c3bb5
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01b40ccf
$ dmesg | grep -i kill
