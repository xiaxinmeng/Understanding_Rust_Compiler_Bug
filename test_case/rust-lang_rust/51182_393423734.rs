plain
[01:05:28] [TIMING] Analysis { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" } -- 2.633
[01:05:28] Dist src
[01:05:39] [TIMING] Src -- 10.642
[01:05:39] Create plain source tarball
[01:06:23] curl: (6) Couldn't resolve host 's3-us-west-1.amazonaws.com'
[01:07:03] curl: (6) Couldn't resolve host 's3-us-west-1.amazonaws.com'
[01:07:43] curl: (6) Couldn't resolve host 's3-us-west-1.amazonaws.com'
[01:07:43] thread 'main' panicked at 'failed to download openssl source: exit code: 6', bootstrap/native.rs:575:17
[01:07:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[01:07:43] Build completed unsuccessfully in 1:02:43
travis_time:end:056ba009:start=1527744597679454020,finish=1527748661807448482,duration=4064127994462

