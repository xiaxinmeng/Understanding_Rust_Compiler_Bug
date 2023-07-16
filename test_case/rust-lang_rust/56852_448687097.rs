plain
travis_time:end:0a3a31cf:start=1545236768042540315,finish=1545236826175942975,duration=58133402660
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=mingw-check
---
travis_time:end:03de210e:start=1545236834213525061,finish=1545236834224200670,duration=10675609
travis_fold:end:before_script.2
travis_fold:start:before_script.3
travis_time:start:0f341c28
$ if [ "$IMAGE" = mingw-check ]; then
    # verify the publish_toolstate script works.
    git clone --depth=1 https://github.com/rust-lang-nursery/rust-toolstate.git;
    cd rust-toolstate;
    python2.7 "$TRAVIS_BUILD_DIR/src/tools/publish_toolstate.py" "$(git rev-parse HEAD)" "$(git log --format=%s -n1 HEAD)" "" "";
Cloning into 'rust-toolstate'...
remote: Enumerating objects: 15, done.
remote: Counting objects:   6% (1/15)   
remote: Counting objects:  13% (2/15)   
---
" exited with 0.
travis_time:start:0ae5e8c0
$ stamp sh -x -c "$RUN_SCRIPT"
[00:00:00] +src/ci/init_repo.sh . /home/travis/rustsrc
[00:00:00] sh: 1: src/ci/init_repo.sh: not found
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 127.
travis_time:start:38c919ee
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Dec 19 16:27:14 UTC 2018
---
travis_time:end:027fa558:start=1545236834974710021,finish=1545236834981613990,duration=6903969
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00b3d0d8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:21815094
travis_time:start:21815094
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:054e4fa8
$ dmesg | grep -i kill
