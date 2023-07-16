plain
travis_time:end:03821390:start=1541098330772788764,finish=1541098332942577510,duration=2169788746
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:37:24]    Compiling parking_lot_core v0.3.0
[00:37:24]    Compiling tempfile v3.0.3
[00:37:25]    Compiling parking_lot v0.6.4
[00:37:26]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:37:29] error[E0599]: no method named `clean` found for type `rustc_data_structures::indexed_vec::IndexVec<rustc_target::abi::VariantIdx, rustc::ty::VariantDef>` in the current scope
[00:37:29]    --> librustdoc/clean/inline.rs:235:48
[00:37:29]     |
[00:37:29] 235 |         variants: cx.tcx.adt_def(did).variants.clean(cx),
[00:37:29]     |
[00:37:29]     = help: items from traits can only be used if the trait is implemented and in scope
[00:37:29]     = help: items from traits can only be used if the trait is implemented and in scope
[00:37:29]     = note: the following trait defines an item `clean`, perhaps you need to implement it:
[00:37:29]             candidate #1: `clean::Clean`
[00:37:31] error: aborting due to previous error
[00:37:31] 
[00:37:31] For more information about this error, try `rustc --explain E0599`.
[00:37:31] error: Could not compile `rustdoc`.
---
[00:37:31] 
[00:37:31] 
[00:37:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:37:31] Build completed unsuccessfully in 0:33:44
[00:37:31] Makefile:28: recipe for target 'all' failed
[00:37:31] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:084af3b0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:1caef410:start=1541100596713997373,finish=1541100596718533753,duration=4536380
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:20c8c929
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1cc50484
travis_time:start:1cc50484
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:258bac93
$ dmesg | grep -i kill
