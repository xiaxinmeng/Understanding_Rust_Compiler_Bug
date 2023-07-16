plain
[CI_JOB_NAME=x86_64-gnu-llvm-5.0]
[00:01:43] [CI_JOB_NAME=x86_64-gnu-llvm-5.0]
[00:01:43] Starting sccache server...
[00:01:43] Traceback (most recent call last):
[00:01:43]   File "/checkout/src/bootstrap/configure.py", line 437, in <module>
[00:01:43]     configure_section(sections[section_key], section_config)
[00:01:43]   File "/checkout/src/bootstrap/configure.py", line 425, in configure_section
travis_time:start:037e92bc
configure: processing command line
[00:01:43] configure: 
[00:01:43] configure: rust.dist-src        := False
---
[00:01:43] configure: llvm.ccache          := sccache
[00:01:43] configure: build.openssl-static := True
[00:01:43] configure: build.build          := x86_64-unknown-linux-gnu
[00:01:43] configure: target.x86_64-unknown-linux-gnu.llvm-config := /usr/lib/llvm-5.0/bin/l ...
[00:01:43] configure: build.missing-tools  := True
[00:01:43] configure: build.configure-args := ['--build=x86_64-unknown-linux-gnu', '--llvm-r ...
[00:01:43]     raise RuntimeError("failed to find config line for {}".format(key))
[00:01:43] RuntimeError: failed to find config line for missing-tools

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:18a1281b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:29dacef6:start=1538332268815093842,finish=1538332268820439291,duration=5345449
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03e75f6b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:013b5450
travis_time:start:013b5450
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a108a8e
$ dmesg | grep -i kill
