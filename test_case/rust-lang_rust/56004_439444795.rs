plain
travis_time:end:16888798:start=1542382461838346583,finish=1542382462878982782,duration=1040636199
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:42:47]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:42:51] error[E0308]: mismatched types
[00:42:51]    --> librustdoc/config.rs:560:21
[00:42:51]     |
[00:42:51] 560 |     Ok(Externs::new(externs))
[00:42:51]     |                     |
[00:42:51]     |                     |
[00:42:51]     |                     expected enum `rustc_data_structures::sorted_map::HybridSortedMap`, found struct `std::collections::BTreeMap`
[00:42:51]     |                     help: try using a variant of the expected type: `rustc_data_structures::sorted_map::HybridSortedMap::Big(externs)`
[00:42:51]     |
[00:42:51]     = note: expected type `rustc_data_structures::sorted_map::HybridSortedMap<std::string::String, std::collections::BTreeSet<std::option::Option<std::string::String>>>`
[00:42:51]                found type `std::collections::BTreeMap<std::string::String, std::collections::BTreeSet<std::option::Option<std::string::String>>>`
[00:42:53] error: aborting due to previous error
[00:42:53] 
[00:42:53] For more information about this error, try `rustc --explain E0308`.
[00:42:53] error: Could not compile `rustdoc`.
---
[00:42:53] 
[00:42:53] 
[00:42:53] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:42:53] Build completed unsuccessfully in 0:38:50
[00:42:53] make: *** [all] Error 1
[00:42:53] Makefile:28: recipe for target 'all' failed
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:26d958fb
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
---
travis_time:end:044ae660:start=1542385051479879057,finish=1542385051484756091,duration=4877034
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1acbed20
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_fa
