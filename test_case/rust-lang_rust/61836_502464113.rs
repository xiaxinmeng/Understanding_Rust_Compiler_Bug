plain
travis_time:end:273ea4f0:start=1560699976089396979,finish=1560700063544742234,duration=87455345255
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:06:40]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:06:44]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:44]    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
[00:07:54]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:08:16] error[E0599]: no method named `attrs_by_hir_id` found for type `&hir::map::Map<'_>` in the current scope
[00:08:16]     --> src/librustc/middle/resolve_lifetime.rs:1593:55
[00:08:16]      |
[00:08:16] 1593 |                                     if self.tcx.hir().attrs_by_hir_id(parent_hir_id).iter()
[00:08:16]      |                                                       ^^^^^^^^^^^^^^^ help: there is a method with a similar name: `get_by_hir_id`
[00:08:26] error: aborting due to previous error
[00:08:26] 
[00:08:26] For more information about this error, try `rustc --explain E0599`.
[00:08:26] error: Could not compile `rustc`.
---
travis_time:end:14092866:start=1560700579630167601,finish=1560700579634682389,duration=4514788
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:203a18b0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:3735c81f
travis_time:start:3735c81f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:032dac38
$ dmesg | grep -i kill
