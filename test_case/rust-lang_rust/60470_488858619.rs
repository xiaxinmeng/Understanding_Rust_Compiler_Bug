plain
travis_time:end:021b2ec0:start=1556836126801678746,finish=1556836127708196578,duration=906517832
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
    100% |████████████████████████████████| 51kB 21.7MB/s 
Collecting colorama<=0.3.9,>=0.2.5 (from awscli)
  Downloading https://files.pythonhosted.org/packages/db/c8/7dcf9dbcb22429512708fe3a547f8b6101c0d02137acbd892505aee57adf/colorama-0.3.9-py2.py3-none-any.whl
Collecting botocore==1.12.141 (from awscli)
  Downloading https://files.pythonhosted.org/packages/af/72/bb5092d4f8a7b6c9a4508b784cdfed6d856e2a202383c345a66da71cc612/botocore-1.12.141-py2.py3-none-any.whl (5.4MB)
    0% |▏                               | 20kB 26.8MB/s eta 0:00:01
    0% |▏                               | 30kB 34.5MB/s eta 0:00:01
    0% |▎                               | 40kB 39.5MB/s eta 0:00:01
    0% |▎                               | 51kB 41.3MB/s eta 0:00:01
---
[00:01:47] 
######################################################################## 100.0%
[00:01:47] extracting /checkout/obj/build/cache/2019-04-11/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:48]     Updating crates.io index
[00:02:04]     Updating git repository `https://github.com/gankro/hashbrown`
[00:02:06]   Downloaded filetime v0.2.4
[00:02:06]   Downloaded cc v1.0.35
[00:02:06]   Downloaded cmake v0.1.38
[00:02:06]   Downloaded toml v0.4.10
---
tidy check
[00:03:57] * 570 error codes
[00:03:57] * highest error code: E0725
[00:03:57] * 254 features
[00:03:58] invalid source: "git+https://github.com/gankro/hashbrown?branch=singleton#09ed47028875a10cb6ad9ffeceb61cc4a5c9d690"
[00:03:58] some tidy checks failed
[00:03:58] 
[00:03:58] 
[00:03:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:58] 
[00:03:58] 
[00:03:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:58] Build completed unsuccessfully in 0:00:45
[00:03:58] Build completed unsuccessfully in 0:00:45
[00:03:58] make: *** [tidy] Error 1
[00:03:58] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0703a43c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu May  2 22:32:57 UTC 2019
---
travis_time:end:0c3d17c4:start=1556836378619553784,finish=1556836378624690117,duration=5136333
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:010d59ac
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2258f832
travis_time:start:2258f832
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:19f08d70
$ dmesg | grep -i kill
