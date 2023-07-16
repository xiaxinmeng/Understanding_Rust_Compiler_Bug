plain
travis_time:end:04144704:start=1544394824014681787,finish=1544394826168454948,duration=2153773161
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:12:02]     --> src/librustc_mir/hair/pattern/_match.rs:1672:13
[00:12:02]      |
[00:12:02] 1672 | /             match *constructor {
[00:12:02] 1673 | |                 Slice(..) => {
[00:12:02] 1674 | |                     // we extract an `Option` for the pointer because slices of zero elements don't
[00:12:02] 1675 | |                     // necessarily point to memory, they are usually just integers. The only time
[00:12:02] 1738 | |                 }
[00:12:02] 1739 | |             }
[00:12:02]      | |_____________^ expected struct `std::vec::Vec`, found struct `smallvec::SmallVec`
[00:12:02]      |
[00:12:02]      |
[00:12:02]      = note: expected type `std::option::Option<std::vec::Vec<&hair::pattern::Pattern<'_>>>`
[00:12:02]                 found type `std::option::Option<smallvec::SmallVec<[&hair::pattern::Pattern<'_>; 2]>>`
[00:12:02] note: match arm with an incompatible type
[00:12:02]      |
[00:12:02] 1733 |                   _ => {
[00:12:02]      |  ______________________^
[00:12:02] 1734 | |                     // If the constructor is a:
[00:12:02] 1734 | |                     // If the constructor is a:
[00:12:02] 1735 | |                     //      Single value: add a row if the constructor equals the pattern.
[00:12:02] 1736 | |                     //      Range: add a row if the constructor contains the pattern.
[00:12:02] 1737 | |                     constructor_intersects_pattern(cx.tcx, constructor, pat)
[00:12:02]      | |_________________^
[00:12:02] 
[00:12:08] error: aborting due to previous error
[00:12:08] 
---
[00:13:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:13:35] expected success, got: exit code: 101
[00:13:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:13:35] Build completed unsuccessfully in 0:10:38
[00:13:35] make: *** [all] Error 1
[00:13:35] Makefile:28: recipe for target 'all' failed
