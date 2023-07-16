plain
travis_time:end:04c9b49f:start=1551797304364543186,finish=1551797424041613477,duration=119677070291
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:end:0039f494:start=1551797429722368573,finish=1551797429727875462,duration=5506889
travis_fold:end:before_install.2
travis_fold:start:before_install.3
travis_time:start:232d296e
$ src/ci/init_gcloud.sh
Skipping initializing the Google Cloud SDK since GCP_CLIENT_SECRET is missing
travis_fold:end:before_install.3
travis_fold:start:before_install.4
travis_time:start:00d2464d
$ if [ "$TRAVIS_OS_NAME" = linux ]; then echo '{"ipv6":true,"fixed-cidr-v6":"fd9a:8454:6789:13f7::/64"}' | sudo tee /etc/docker/daemon.json; sudo service docker restart; fi
---
[00:00:31] +src/ci/docker/run.sh x86_64-gnu-llvm-6.0
[00:00:32] travis_time:end:0f5875e0:start=1551797432417998225,finish=1551797464205597496,duration=31787599271
travis_fold:start:build_docker
travis_time:start:18913a48
Attempting to download gs://rust-lang-ci-cache/docker/7dda861d845e0d7fdd5b31814b6da2e81399ab970735c4c24a053bc7a7cf1b5a86bbc7db27aa36833130c2d20939136d930244b601233ad6359a7fdccfc6edec
[00:00:32]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:00:32]                                  Dload  Upload   Total   Spent    Left  Speed
[00:00:32] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[00:01:42]  ---> 30111bdd5be1
[00:01:42] Successfully built 30111bdd5be1
[00:01:42] Successfully tagged rust-ci:latest
[00:01:42] Built container sha256:30111bdd5be12e12e4356536809746dd0dbef11bba927509c2080f82dcd229fc
[00:01:42] Uploading finished image to gs://rust-lang-ci-cache/docker/7dda861d845e0d7fdd5b31814b6da2e81399ab970735c4c24a053bc7a7cf1b5a86bbc7db27aa36833130c2d20939136d930244b601233ad6359a7fdccfc6edec
[00:01:42] src/ci/docker/run.sh: line 64: /home/travis/google-cloud-sdk/bin/gsutil: No such file or directory
[00:01:55] xargs: docker: terminated by signal 13

[00:01:55] travis_time:end:18913a48:start=1551797464219240230,finish=1551797547565470383,duration=83346230153
[CI_JOB_NAME=x86_64-gnu-llvm-6.0]
[00:01:55] [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
---

[00:04:09] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:09] tidy error: /checkout/src/ci/init_gcloud.sh:9: line longer than 100 chars
[00:04:09] tidy error: /checkout/src/ci/init_gcloud.sh:16: line longer than 100 chars
[00:04:10] some tidy checks failed
[00:04:10] 
[00:04:10] 
[00:04:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:10] 
[00:04:10] 
[00:04:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:10] Build completed unsuccessfully in 0:00:43
[00:04:10] Build completed unsuccessfully in 0:00:43
[00:04:10] Makefile:68: recipe for target 'tidy' failed
[00:04:10] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:157489a5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar  5 14:54:43 UTC 2019
---
travis_time:end:014e66d0:start=1551797684621878728,finish=1551797684626574703,duration=4695975
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:053b458c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:12c292ca
travis_time:start:12c292ca
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02748ce4
$ dmesg | grep -i kill
