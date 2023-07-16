plain
travis_time:end:0ace4988:start=1548690154374732301,finish=1548690156663000130,duration=2288267829
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=mingw-check
---
[00:02:44] configure: 
[00:02:44] configure: rust.experimental-parallel-queries := True
[00:02:44] configure: build.configure-args := ['--enable-experimental-parallel-queries']
[00:02:44] Traceback (most recent call last):
[00:02:44]   File "/checkout/src/bootstrap/configure.py", line 442, in <module>
[00:02:44]     configure_section(sections[section_key], section_config)
[00:02:44]   File "/checkout/src/bootstrap/configure.py", line 430, in configure_section
[00:02:44]     raise RuntimeError("failed to find config line for {}".format(key))
travis_time:end:1cf53428:start=1548690166342216943,finish=1548690330794643119,duration=164452426176
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:000bf602
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0122b180:start=1548690331489342016,finish=1548690331493576869,duration=4234853
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00dafe44
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:128d9714
travis_time:start:128d9714
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1fb98aba
$ dmesg | grep -i kill
