plain
travis_time:end:1626b532:start=1553446556262267663,finish=1553446630332963213,duration=74070695550
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:05:53]     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
[00:05:54]     Checking rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
[00:05:54]     Checking rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
[00:05:55]     Checking rustc_interface v0.0.0 (/checkout/src/librustc_interface)
[00:05:56] error[E0277]: `(dyn rustc::middle::cstore::MetadataLoader + std::marker::Sync + 'static)` cannot be sent between threads safely
[00:05:56]    --> src/librustc_interface/passes.rs:993:13
[00:05:56] 993 |             cstore,
[00:05:56] 993 |             cstore,
[00:05:56]     |             ^^^^^^ `(dyn rustc::middle::cstore::MetadataLoader + std::marker::Sync + 'static)` cannot be sent between threads safely
[00:05:56]     |
[00:05:56]     = help: the trait `std::marker::Send` is not implemented for `(dyn rustc::middle::cstore::MetadataLoader + std::marker::Sync + 'static)`
[00:05:56]     = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<(dyn rustc::middle::cstore::MetadataLoader + std::marker::Sync + 'static)>`
[00:05:56]     = note: required because it appears within the type `std::boxed::Box<(dyn rustc::middle::cstore::MetadataLoader + std::marker::Sync + 'static)>`
[00:05:56]     = note: required because it appears within the type `rustc_metadata::cstore::CStore`
[00:05:56]     = note: required because of the requirements on the impl of `std::marker::Sync` for `std::sync::Arc<rustc_metadata::cstore::CStore>`
[00:05:56]     = note: required for the cast to the object type `dyn std::any::Any + std::marker::Sync`
[00:05:56] error: aborting due to previous error
[00:05:56] 
[00:05:56] For more information about this error, try `rustc --explain E0277`.
[00:05:56] error: Could not compile `rustc_interface`.
---
travis_time:end:183e0560:start=1553446999471825974,finish=1553446999476831941,duration=5005967
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:090cf8ba
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07d68854
travis_time:start:07d68854
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00643665
$ dmesg | grep -i kill
