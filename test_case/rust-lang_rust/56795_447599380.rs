plain
travis_time:end:19718b0b:start=1544904461251177531,finish=1544904516443251849,duration=55192074318
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=mingw-check
---
[00:05:52]     Checking parking_lot_core v0.3.0
[00:05:53]     Checking parking_lot v0.6.4
[00:05:53]     Checking tempfile v3.0.5
[00:05:54]     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:05:54] error[E0463]: can't find crate for `serde_derive` which `rustc` depends on
[00:05:54]    |
[00:05:54] 34 | extern crate rustc;
[00:05:54]    | ^^^^^^^^^^^^^^^^^^^ can't find crate
[00:05:54] 
[00:05:54] 
[00:05:54] error: aborting due to previous error
[00:05:54] 
[00:05:54] For more information about this error, try `rustc --explain E0463`.
[00:05:54] error: Could not compile `rustdoc`.
[00:05:54] 
[00:05:54] To learn more, run the command again with --verbose.
[00:05:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "--message-format" "json"
[00:05:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
[00:05:54] Build completed unsuccessfully in 0:04:58
travis_time:end:292d0844:start=1544904524653343983,finish=1544904879353194240,duration=354699850257
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
---
travis_time:end:1b520126:start=1544904879764218272,finish=1544904879768763118,duration=4544846
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:122805cf
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:26bd2188
travis_time:start:26bd2188
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:054ef346
$ dmesg | grep -i kill
