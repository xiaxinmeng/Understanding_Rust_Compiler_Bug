plain
travis_time:end:200fe7c8:start=1544892482194715200,finish=1544892484226652228,duration=2031937028
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ bash -c 'echo $BASH_VERSION'
4.3.11(1)-release
---
  fi
travis_time:end:0740c79b:start=1544892484537715732,finish=1544892484552173252,duration=14457520
travis_fold:end:before_script.2
travis_time:start:16efc990
$ git clone --depth=1 https://github.com/rust-lang-nursery/rust-toolstate.git; python2.7 "$TRAVIS_BUILD_DIR/src/tools/publish_toolstate.py" "$(git rev-parse HEAD)" "$(git log --format=%s -n1 HEAD)" "" "";
remote: Enumerating objects: 15, done.
remote: Counting objects:   6% (1/15)   
remote: Counting objects:  13% (2/15)   
remote: Counting objects:  20% (3/15)   
---
Unpacking objects:  93% (14/15)   
Unpacking objects: 100% (15/15)   
Unpacking objects: 100% (15/15), done.
Traceback (most recent call last):
  File "/home/travis/build/rust-lang/rust/src/tools/publish_toolstate.py", line 126, in <module>
    cur_datetime
  File "/home/travis/build/rust-lang/rust/src/tools/publish_toolstate.py", line 57, in update_latest
    with open('_data/latest.json', 'rb+') as f:
IOError: [Errno 2] No such file or directory: '_data/latest.json'
travis_time:end:16efc990:start=1544892484559641094,finish=1544892485298679099,duration=739038005
The command "git clone --depth=1 https://github.com/rust-lang-nursery/rust-toolstate.git; python2.7 "$TRAVIS_BUILD_DIR/src/tools/publish_toolstate.py" "$(git rev-parse HEAD)" "$(git log --format=%s -n1 HEAD)" "" "";" exited with 1.
travis_time:start:00a853fc
---
travis_time:end:0dee69ef:start=1544892485392756141,finish=1544892485400085264,duration=7329123
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09a6bfa2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09f1c51a
travis_time:start:09f1c51a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0d1a1be6
$ dmesg | grep -i kill
