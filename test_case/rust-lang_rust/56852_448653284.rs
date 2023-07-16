plain
travis_time:end:06bf03f4:start=1545235370378713579,finish=1545235444940786208,duration=74562072629
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=mingw-check
---
travis_time:end:0984a2ce:start=1545235455047729657,finish=1545235455058689964,duration=10960307
travis_fold:end:before_script.2
travis_fold:start:before_script.3
travis_time:start:1c339a54
$ if [ "$IMAGE" = mingw-check ]; then
    # verify the publish_toolstate script works.
    git clone --depth=1 https://github.com/rust-lang-nursery/rust-toolstate.git;
    cd rust-toolstate;
    python2.7 "$TRAVIS_BUILD_DIR/src/tools/publish_toolstate.py?" "$(git rev-parse HEAD)" "$(git log --format=%s -n1 HEAD)" "" "";
Cloning into 'rust-toolstate'...
remote: Enumerating objects: 15, done.
remote: Counting objects:   6% (1/15)   
remote: Counting objects:  13% (2/15)   
---
Unpacking objects:  86% (13/15)   
Unpacking objects:  93% (14/15)   
Unpacking objects: 100% (15/15)   
Unpacking objects: 100% (15/15), done.
python2.7: can't open file '/home/travis/build/rust-lang/rust/src/tools/publish_toolstate.py?': [Errno 2] No such file or directory
travis_time:end:1c339a54:start=1545235455065095855,finish=1545235455630669273,duration=565573418
The command "if [ "$IMAGE" = mingw-check ]; then
    # verify the publish_toolstate script works.
    git clone --depth=1 https://github.com/rust-lang-nursery/rust-toolstate.git;
    cd rust-toolstate;
    python2.7 "$TRAVIS_BUILD_DIR/src/tools/publish_toolstate.py?" "$(git rev-parse HEAD)" "$(git log --format=%s -n1 HEAD)" "" "";
  " failed and exited with 2 during .
Your build has been stopped.
