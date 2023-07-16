plain
travis_time:end:0911bacb:start=1558736622237679217,finish=1558736710156333223,duration=87918654006
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:21:17]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:21:26] error[E0308]: mismatched types
[00:21:26]    --> src/librustc_mir/dataflow/move_paths/builder.rs:160:47
[00:21:26]     |
[00:21:26] 160 |                               Place::Projection(PlaceProjection {
[00:21:26]     |  _______________________________________________^
[00:21:26] 161 | |                                 base: proj.base,
[00:21:26] 162 | |                                 elem: proj.elem.clone(),
[00:21:26] 163 | |                             }),
[00:21:26]     | |_____________________________^ expected struct `std::boxed::Box`, found struct `rustc::mir::Projection`
[00:21:26]     |
[00:21:26]     = note: expected type `std::boxed::Box<rustc::mir::Projection<rustc::mir::Place<'_>, _, &rustc::ty::TyS<'_>>>`
[00:21:26]                found type `rustc::mir::Projection<rustc::mir::Place<'_>, _, &rustc::ty::TyS<'_>>`
[00:21:32] error: aborting due to previous error
[00:21:32] 
[00:21:32] For more information about this error, try `rustc --explain E0308`.
[00:21:32] error: Could not compile `rustc_mir`.
---
travis_time:end:0b4c94e1:start=1558738181822378342,finish=1558738181827317822,duration=4939480
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0e3150e8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:39cbd822
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/
