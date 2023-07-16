plain
travis_time:end:180e89aa:start=1543960784351667664,finish=1543960785390121340,duration=1038453676
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
    100% |████████████████████████████████| 1.4MB 845kB/s 
Collecting botocore==1.12.59 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/21/cf/71dfc14692883aaf709bc1098a56770173a760a14b0b1cb74471609181be/botocore-1.12.59-py2.py3-none-any.whl (5.1MB)
    0% |                                | 10kB 35.6MB/s eta 0:00:01
    0% |▏                               | 20kB 31.7MB/s eta 0:00:01
    0% |▏                               | 30kB 36.8MB/s eta 0:00:01
    0% |▎                               | 40kB 28.9MB/s eta 0:00:01
---
###############################################################           88.2%
######################################################################## 100.0%
[00:01:21] extracting /checkout/obj/build/cache/2018-10-30/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:21]     Updating crates.io index
[00:01:28] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:28] Build completed unsuccessfully in 0:00:32
[00:01:28] make: *** [prepare] Error 1
[00:01:28] Makefile:81: recipe for target 'prepare' failed
[00:01:29] Command failed. Attempt 2/5:
[00:01:29] Command failed. Attempt 2/5:
[00:01:29]     Updating crates.io index
[00:01:30] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:30] Build completed unsuccessfully in 0:00:00
[00:01:30] Makefile:81: recipe for target 'prepare' failed
[00:01:30] make: *** [prepare] Error 1
[00:01:32] Command failed. Attempt 3/5:
[00:01:32] Command failed. Attempt 3/5:
[00:01:32]     Updating crates.io index
[00:01:32] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:32] Build completed unsuccessfully in 0:00:00
[00:01:32] Makefile:81: recipe for target 'prepare' failed
[00:01:32] make: *** [prepare] Error 1
[00:01:35] Command failed. Attempt 4/5:
[00:01:35] Command failed. Attempt 4/5:
[00:01:35]     Updating crates.io index
[00:01:36] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:36] Build completed unsuccessfully in 0:00:00
[00:01:36] make: *** [prepare] Error 1
[00:01:36] Makefile:81: recipe for target 'prepare' failed
[00:01:40] Command failed. Attempt 5/5:
[00:01:40] Command failed. Attempt 5/5:
[00:01:40]     Updating crates.io index
[00:01:41] error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
[00:01:41] Build completed unsuccessfully in 0:00:00
[00:01:41] make: *** [prepare] Error 1
[00:01:41] Makefile:81: recipe for target 'prepare' failed
[00:01:41] The command has failed after 5 attempts.
---
travis_time:end:0ed12b15:start=1543960896617051379,finish=1543960896622972360,duration=5920981
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:017b0d3a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:009aa5a3
travis_time:start:009aa5a3
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:191a4f50
$ dmesg | grep -i kill
