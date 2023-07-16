plain
travis_time:end:0621b47c:start=1545234562056265895,finish=1545234617808945632,duration=55752679737
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:end:06b85b14:start=1545234626668780115,finish=1545234626679224666,duration=10444551
travis_fold:end:before_script.2
travis_fold:start:before_script.3
travis_time:start:12f7f71a
$ # verify the publish_toolstate script works. if [ "$IMAGE" = mingw-check ]; then
    git clone --depth=1 https://github.com/rust-lang-nursery/rust-toolstate.git;
    cd rust-toolstate;
    python2.7 "$TRAVIS_BUILD_DIR/src/tools/publish_toolstate.py" "$(git rev-parse HEAD)" "$(git log --format=%s -n1 HEAD)" "" "";
Cloning into 'rust-toolstate'...
remote: Enumerating objects: 15, done.
remote: Counting objects:   6% (1/15)   
remote: Counting objects:  13% (2/15)   
---
remote: Total 15 (delta 1), reused 9 (delta 0), pack-reused 0
Unpacking objects:  93% (14/15)   
Unpacking objects: 100% (15/15)   
Unpacking objects: 100% (15/15), done.
<Nothing changed>
/home/travis/.travis/job_stages: eval: line 108: syntax error near unexpected token `fi'
/home/travis/.travis/job_stages: eval: line 108: `  fi'
travis_time:end:12f7f71a:start=1545234626686009807,finish=1545234627240123325,duration=554113518
The command "# verify the publish_toolstate script works. if [ "$IMAGE" = mingw-check ]; then
    git clone --depth=1 https://github.com/rust-lang-nursery/rust-toolstate.git;
    cd rust-toolstate;
    python2.7 "$TRAVIS_BUILD_DIR/src/tools/publish_toolstate.py" "$(git rev-parse HEAD)" "$(git log --format=%s -n1 HEAD)" "" "";
  " failed and exited with 1 during .
Your build has been stopped.
