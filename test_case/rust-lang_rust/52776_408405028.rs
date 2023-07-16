plain
[00:03:44] Successfully tagged rust-ci:latest
[00:03:44] Built container sha256:18ee8e1bf22bb78b8515334c9d0d86da7f051eeed99fcd60571dd776c406dccf
[00:03:44] Uploading finished image to s3://rust-lang-ci-sccache2/docker/de94e11fb622ebd51292930689046e4dcd653833568b97f8dbcebaae8181ce5ef0a7935c29e835bdd3f7ec37b1c23cf77eb4686cd315c7beefdafb75370cd2ba
[00:03:44] 
[00:03:44] Partial credentials found in env, missing: AWS_SECRET_ACCESS_KEY
[00:03:51] xargs: docker: terminated by signal 13

[00:03:52] travis_time:end:227617fa:start=1532694432787930234,finish=1532694560135557156,duration=127347626922
[CI_JOB_NAME=x86_64-gnu-llvm-3.9]
[00:03:52] [CI_JOB_NAME=x86_64-gnu-llvm-3.9]
---
[00:04:10] curl: (22) The requested URL returned error: 404 Not Found
[00:04:10] 
[00:04:10] spurious failure, trying again
[00:04:11] curl: (22) The requested URL returned error: 404 Not Found
[00:04:11] failed to run: curl -s --retry 3 -Sf -o /tmp/tmpIlfoGZ.sha256 https://static.rust-lang.org/dist/2018-07-20/cargo-1.27.0-x86_64-unknown-linux-gnu.tar.gz.sha256
[00:04:11] make: *** [prepare] Error 1
[00:04:11] Makefile:81: recipe for target 'prepare' failed
[00:04:12] Command failed. Attempt 2/5:
[00:04:12] curl: (22) The requested URL returned error: 404 Not Found
---
[00:04:13] curl: (22) The requested URL returned error: 404 Not Found
[00:04:13] 
[00:04:13] spurious failure, trying again
[00:04:13] curl: (22) The requested URL returned error: 404 Not Found
[00:04:13] failed to run: curl -s --retry 3 -Sf -o /tmp/tmpLzGExT.sha256 https://static.rust-lang.org/dist/2018-07-20/cargo-1.27.0-x86_64-unknown-linux-gnu.tar.gz.sha256
[00:04:13] Makefile:81: recipe for target 'prepare' failed
[00:04:13] make: *** [prepare] Error 1
[00:04:15] Command failed. Attempt 3/5:
[00:04:15] curl: (22) The requested URL returned error: 404 Not Found
---
[00:04:16] curl: (22) The requested URL returned error: 404 Not Found
[00:04:16] 
[00:04:16] spurious failure, trying again
[00:04:16] curl: (22) The requested URL returned error: 404 Not Found
[00:04:16] failed to run: curl -s --retry 3 -Sf -o /tmp/tmpdAxjLp.sha256 https://static.rust-lang.org/dist/2018-07-20/cargo-1.27.0-x86_64-unknown-linux-gnu.tar.gz.sha256
[00:04:16] Makefile:81: recipe for target 'prepare' failed
[00:04:16] make: *** [prepare] Error 1
[00:04:19] Command failed. Attempt 4/5:
[00:04:20] curl: (22) The requested URL returned error: 404 Not Found
---
[00:04:21] curl: (22) The requested URL returned error: 404 Not Found
[00:04:21] 
[00:04:21] spurious failure, trying again
[00:04:21] curl: (22) The requested URL returned error: 404 Not Found
[00:04:21] failed to run: curl -s --retry 3 -Sf -o /tmp/tmpdihlHx.sha256 https://static.rust-lang.org/dist/2018-07-20/cargo-1.27.0-x86_64-unknown-linux-gnu.tar.gz.sha256
[00:04:21] Makefile:81: recipe for target 'prepare' failed
[00:04:21] make: *** [prepare] Error 1
[00:04:25] Command failed. Attempt 5/5:
[00:04:25] curl: (22) The requested URL returned error: 404 Not Found
---
[00:04:26] curl: (22) The requested URL returned error: 404 Not Found
[00:04:26] 
[00:04:26] spurious failure, trying again
[00:04:26] curl: (22) The requested URL returned error: 404 Not Found
[00:04:26] failed to run: curl -s --retry 3 -Sf -o /tmp/tmp9V8i5E.sha256 https://static.rust-lang.org/dist/2018-07-20/cargo-1.27.0-x86_64-unknown-linux-gnu.tar.gz.sha256
[00:04:26] make: *** [prepare] Error 1
[00:04:26] Makefile:81: recipe for target 'prepare' failed
[00:04:26] The command has failed after 5 attempts.
travis_time:end:1a8f2e43:start=1532694328276400843,finish=1532694595184738366,duration=266908337523
---
travis_time:end:01c66438:start=1532694595463149788,finish=1532694595471291014,duration=8141226
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07001da4
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05ee7af4
$ dmesg | grep -i kill
