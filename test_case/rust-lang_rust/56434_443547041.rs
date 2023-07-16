plain
travis_time:end:124fcd3a:start=1543785306787115816,finish=1543785360276971360,duration=53489855544
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:01:13] Successfully built 70981647206b
[00:01:13] Successfully tagged rust-ci:latest
[00:01:13] Built container sha256:70981647206b232576a889bdaad1512823623c45901cd994252c34ba957d030b
[00:01:13] Uploading finished image to s3://rust-lang-ci-sccache2/docker/74b3db5e1a3f47588052701a2d4e2019f028a5a349ddce282ebe42cbaba480c22235c657c63a125b1e21cf77cdad928e69c660393bd9772ac119139c9934981d
[00:01:56] upload failed: - to s3://rust-lang-ci-sccache2/docker/74b3db5e1a3f47588052701a2d4e2019f028a5a349ddce282ebe42cbaba480c22235c657c63a125b1e21cf77cdad928e69c660393bd9772ac119139c9934981d Unable to locate credentials

[00:01:56] travis_time:end:27793b50:start=1543785379857906141,finish=1543785485573753997,duration=105715847856
[CI_JOB_NAME=x86_64-gnu-llvm-5.0]
[00:01:56] [CI_JOB_NAME=x86_64-gnu-llvm-5.0]
---
[00:52:36] .............................................................................i...................... 1900/2924
[00:52:54] ................................................i................................................... 2000/2924
[00:53:14] .................................................................................................... 2100/2924
[00:53:28] .................................................................................................... 2200/2924
[00:53:37] ..............................................ii....................F............................... 2300/2924
[00:54:02] .................................................................................................... 2500/2924
[00:54:33] .................................................................................................... 2600/2924
[00:54:42] .................................................................................................... 2700/2924
[00:54:52] .................................................................................................... 2800/2924
---
travis_time:end:0851ab53:start=1543788677389523108,finish=1543788677393382688,duration=3859580
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0b00eb4e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
