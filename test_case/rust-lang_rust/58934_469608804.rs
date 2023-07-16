plain
travis_time:end:0026de30:start=1551777639174242737,finish=1551777716466066037,duration=77291823300
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:end:02a53eb0:start=1551777721660441944,finish=1551777721665394336,duration=4952392
travis_fold:end:before_install.2
travis_fold:start:before_install.3
travis_time:start:072f76ca
$ src/ci/init_gcloud.sh
Skipping initializing the Google Cloud SDK since GCP_CLIENT_SECRET is missing
travis_fold:end:before_install.3
travis_fold:start:before_install.4
travis_time:start:20235ef7
$ if [ "$TRAVIS_OS_NAME" = linux ]; then echo '{"ipv6":true,"fixed-cidr-v6":"fd9a:8454:6789:13f7::/64"}' | sudo tee /etc/docker/daemon.json; sudo service docker restart; fi
---
[00:00:16] +src/ci/docker/run.sh x86_64-gnu-llvm-6.0
[00:00:16] travis_time:end:01c766b8:start=1551777724403449877,finish=1551777741080553607,duration=16677103730
travis_fold:start:build_docker
travis_time:start:20a726d6
Attempting to download gs://rust-lang-ci-cache/docker/7dda861d845e0d7fdd5b31814b6da2e81399ab970735c4c24a053bc7a7cf1b5a86bbc7db27aa36833130c2d20939136d930244b601233ad6359a7fdccfc6edec
[00:00:16]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:00:16]                                  Dload  Upload   Total   Spent    Left  Speed
[00:00:17] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[00:01:35]  ---> 2e319758c598
[00:01:35] Successfully built 2e319758c598
[00:01:35] Successfully tagged rust-ci:latest
[00:01:35] Built container sha256:2e319758c598561aa03d74fed257895d34db5b692561ac735f1f767efa15c6bc
[00:01:35] Uploading finished image to gs://rust-lang-ci-cache/docker/7dda861d845e0d7fdd5b31814b6da2e81399ab970735c4c24a053bc7a7cf1b5a86bbc7db27aa36833130c2d20939136d930244b601233ad6359a7fdccfc6edec
[00:01:35] src/ci/docker/run.sh: line 64: /home/travis/google-cloud-sdk/bin/gsutil: No such file or directory
[00:01:51] xargs: docker: terminated by signal 13

[00:01:52] travis_time:end:20a726d6:start=1551777741092281216,finish=1551777836131846333,duration=95039565117
[CI_JOB_NAME=x86_64-gnu-llvm-6.0]
[00:01:52] [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
---

[00:04:14] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:15] tidy error: /checkout/src/ci/init_gcloud.sh:9: line longer than 100 chars
[00:04:15] tidy error: /checkout/src/ci/init_gcloud.sh:16: line longer than 100 chars
[00:04:16] some tidy checks failed
[00:04:16] 
[00:04:16] 
[00:04:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:16] 
[00:04:16] 
[00:04:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:16] Build completed unsuccessfully in 0:00:44
[00:04:16] Build completed unsuccessfully in 0:00:44
[00:04:16] Makefile:68: recipe for target 'tidy' failed
[00:04:16] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:27e95a9d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar  5 09:26:21 UTC 2019
---
travis_time:end:2b133148:start=1551777982116030412,finish=1551777982120741697,duration=4711285
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01de2e10
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08f53b16
travis_time:start:08f53b16
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:17eb467a
$ dmesg | grep -i kill
