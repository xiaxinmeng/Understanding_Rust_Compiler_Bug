plain
travis_fold:end:before_install.6
travis_fold:start:before_install.7
travis_time:start:318d59d2
$ sudo grep -E 'google-clock-skew|ntpd|startup-script' /var/log/syslog || true
Aug  6 22:27:11 travis-job-164e325b-14b6-47fc-be70-595199156787 google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug  6 22:27:11 travis-job-164e325b-14b6-47fc-be70-595199156787 google-clock-skew: INFO Clock drift token has changed: 0.
Aug  6 22:27:12 travis-job-164e325b-14b6-47fc-be70-595199156787 google-clock-skew: INFO Synced system time with hardware clock.
Aug  6 22:27:18 travis-job-164e325b-14b6-47fc-be70-595199156787 ntpdate[974]: adjust time server 169.254.169.254 offset 0.006899 sec
Aug  6 22:27:35 travis-job-164e325b-14b6-47fc-be70-595199156787 ntpdate[1871]: adjust time server 169.254.169.254 offset 0.002095 sec
Aug  6 22:27:42 travis-job-164e325b-14b6-47fc-be70-595199156787 ntpd[1907]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug  6 22:27:42 travis-job-164e325b-14b6-47fc-be70-595199156787 ntpd[1908]: proto: precision = 0.105 usec
Aug  6 22:27:42 travis-job-164e325b-14b6-47fc-be70-595199156787 ntpd[1908]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug  6 22:27:42 travis-job-164e325b-14b6-47fc-be70-595199156787 ntpd[1908]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug  6 22:27:42 travis-job-164e325b-14b6-47fc-be70-595199156787 ntpd[1908]: Listen and drop on 1 v6wildcard :: UDP 123
Aug  6 22:27:42 travis-job-164e325b-14b6-47fc-be70-595199156787 ntpd[1908]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug  6 22:27:42 travis-job-164e325b-14b6-47fc-be70-595199156787 ntpd[1908]: Listen normally on 3 eth0 10.20.2.127 UDP 123
Aug  6 22:27:42 travis-job-164e325b-14b6-47fc-be70-595199156787 ntpd[1908]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug  6 22:27:42 travis-job-164e325b-14b6-47fc-be70-595199156787 ntpd[1908]: peers refreshed
Aug  6 22:27:42 travis-job-164e325b-14b6-47fc-be70-595199156787 ntpd[1908]: Listening on routing socket on fd #21 for interface updates
Aug  6 22:27:42 travis-job-164e325b-14b6-47fc-be70-595199156787 startup-script: INFO Starting startup scripts.
Aug  6 22:27:42 travis-job-164e325b-14b6-47fc-be70-595199156787 startup-script: INFO Found startup-script in metadata.
Aug  6 22:27:42 travis-job-164e325b-14b6-47fc-be70-595199156787 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug  6 22:27:42 travis-job-164e325b-14b6-47fc-be70-595199156787 startup-script: INFO startup-script: job 1 at Tue Aug  7 01:37:00 2018
Aug  6 22:27:42 travis-job-164e325b-14b6-47fc-be70-595199156787 startup-script: INFO startup-script: Return code 0.
Aug  6 22:27:42 travis-job-164e325b-14b6-47fc-be70-595199156787 startup-script: INFO Finished running startup scripts.
travis_fold:end:before_install.7
travis_fold:start:install
travis_time:start:00a8eb06
$ case "$TRAVIS_OS_NAME" in linux) travis_retry curl -fo $HOME/stamp https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2017-03-17-stamp-x86_64-unknown-linux-musl && chmod +x $HOME/stamp && export PATH=$PATH:$HOME ;; osx) if [[ "$RUST_CHECK_TARGET" == dist ]]; then travis_retry brew update && travis_retry brew install xz; fi && travis_retry curl -fo /usr/local/bin/sccache https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2018-04-02-sccache-x86_64-apple-darwin && chmod +x /usr/local/bin/sccache && travis_retry curl -fo /usr/local/bin/stamp https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2017-03-17-stamp-x86_64-apple-darwin && chmod +x /usr/local/bin/stamp && travis_retry curl -f http://releases.llvm.org/6.0.0/clang+llvm-6.0.0-x86_64-apple-darwin.tar.xz | tar xJf - && export CC=`pwd`/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang && export CXX=`pwd`/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang++ && export AR=ar ;; esac
---
[00:44:03] ....................................................................................................
[00:44:05] ....................................................................................................
[00:44:07] ....................................................................................................
[00:44:10] ....................................................................................................
[00:44:13] .....iiiiiiiii......................................................................................
[00:44:19] ....................................................................................................
[00:44:22] ..........i.........................................................................................
[00:44:25] ...................i................................................................................
[00:44:28] ....................................................................................................
---
[00:50:06] 
[00:50:06] running 1886 tests
[00:50:08] ....................................................................................................
[00:50:11] ....................................................................................................
[00:50:13] .............................................................................................F......
[00:50:20] ...........................................................................................i........
[00:50:22] ...................................................ii.iii...........................................
[00:50:25] ...................................................................................i................
[00:50:28] ............................i.......................................................................
---
[00:50:58] failures:
[00:50:58] 
[00:50:58] ---- [compile-fail] compile-fail/borrowck/borrowck-thread-local-static-borrow-outlives-fn.rs#mir stdout ----
[00:50:58] 
[00:50:58] error in revision `mir`: /checkout/src/test/compile-fail/borrowck/borrowck-thread-local-static-borrow-outlives-fn.rs:21: unexpected error: '21:20: 21:24: thread-local variable borrowed past end of function [E0712]'
[00:50:58] 
[00:50:58] error in revision `mir`: /checkout/src/test/compile-fail/borrowck/borrowck-thread-local-static-borrow-outlives-fn.rs:21: expected error not found: [E0597]
[00:50:58] error in "--color" "always"
[00:50:58] expected success, got: exit code: 101
[00:50:58] 
[00:50:58] 
[00:50:58] 
[00:50:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:50:58] Build completed unsuccessfully in 0:09:27
[00:50:58] Makefile:58: recipe for target 'check' failed
[00:50:58] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1661e25f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:15985f05:start=1533597608076439349,finish=1533597608144026849,duration=67587500
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c286e40
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00b4f780
$ dmesg | grep -i kill
