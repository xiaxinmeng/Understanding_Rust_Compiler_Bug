plain
travis_time:end:3117fd20:start=1558733688206406608,finish=1558733776400721496,duration=88194314888
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
    99% |████████████████████████████████| 542kB 46.3MB/s eta 0:00:01
    100% |████████████████████████████████| 552kB 22.2MB/s 
Requirement already satisfied: PyYAML<=3.13,>=3.10 in /usr/lib/python2.7/dist-packages (from awscli) (3.11)
Collecting botocore==1.12.156 (from awscli)
  Downloading https://files.pythonhosted.org/packages/f5/aa/79fc47ccc3c7d0f36aafb9d85091d7d8a8f10d8ad24ccf3a89cf126b9f4e/botocore-1.12.156-py2.py3-none-any.whl (5.4MB)
    0% |▏                               | 20kB 26.5MB/s eta 0:00:01
    0% |▏                               | 30kB 30.4MB/s eta 0:00:01
    0% |▎                               | 40kB 32.4MB/s eta 0:00:01
    0% |▎                               | 51kB 35.6MB/s eta 0:00:01
---
[00:14:43]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:14:57] error[E0308]: mismatched types
[00:14:57]    --> src/librustc_mir/interpret/place.rs:636:78
[00:14:57]     |
[00:14:57] 636 |                 PlaceBase::Static(place_static) => self.eval_place_to_mplace(place_static)?.into(),
[00:14:57]     |
[00:14:57]     = note: expected type `&rustc::mir::Place<'_>`
[00:14:57]     = note: expected type `&rustc::mir::Place<'_>`
[00:14:57]                found type `&std::boxed::Box<rustc::mir::Static<'_>>`
[00:15:00] error: aborting due to previous error
[00:15:00] 
[00:15:00] For more information about this error, try `rustc --explain E0308`.
[00:15:00] error: Could not compile `rustc_mir`.
---
travis_time:end:0e6d5203:start=1558734849400642168,finish=1558734849405298164,duration=4655996
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:10b20adb
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00d8f606
travis_time:start:00d8f606
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01d07e1f
$ dmesg | grep -i kill
