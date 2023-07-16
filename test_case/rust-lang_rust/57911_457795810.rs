plain
travis_time:end:1dae3b09:start=1548470747299907110,finish=1548470826173477502,duration=78873570392
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
    97% |███████████████████████████████▍| 532kB 41.8MB/s eta 0:00:01
    99% |████████████████████████████████| 542kB 41.5MB/s eta 0:00:01
    100% |████████████████████████████████| 552kB 22.5MB/s 
Collecting botocore==1.12.86 (from awscli)
  Downloading https://files.pythonhosted.org/packages/d7/af/fd9c0f1f0fdc03d3367a56f35093f8b1020ba1a97ead9fa580156895944b/botocore-1.12.86-py2.py3-none-any.whl (5.2MB)
    0% |▏                               | 20kB 25.1MB/s eta 0:00:01
    0% |▏                               | 30kB 29.1MB/s eta 0:00:01
    0% |▎                               | 40kB 28.6MB/s eta 0:00:01
    0% |▎                               | 51kB 29.2MB/s eta 0:00:01
---
[00:13:44]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:13:53] error[E0061]: this function takes 2 parameters but 3 parameters were supplied
[00:13:53]   --> src/librustc_mir/hair/constant.rs:44:22
[00:13:53]    |
[00:13:53] 44 |                 val: ConstValue::new_slice(Scalar::Ptr(id.into()), s.len() as u64, &tcx),
[00:13:53] 
[00:13:59] error: aborting due to previous error
[00:13:59] 
[00:13:59] For more information about this error, try `rustc --explain E0061`.
[00:13:59] For more information about this error, try `rustc --explain E0061`.
[00:13:59] error: Could not compile `rustc_mir`.
[00:13:59] warning: build failed, waiting for other jobs to finish...
[00:16:43] error: build failed
[00:16:43] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:16:43] expected success, got: exit code: 101
[00:16:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:16:43] Build completed unsuccessfully in 0:13:15
[00:16:43] make: *** [all] Error 1
[00:16:43] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:3b258860
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Jan 26 03:03:58 UTC 2019
