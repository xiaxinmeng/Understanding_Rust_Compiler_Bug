plain
travis_time:end:0161159c:start=1543030363132812780,finish=1543030365296056885,duration=2163244105
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:12:59]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:13:02] error[E0308]: mismatched types
[00:13:02]    --> src/librustc_typeck/astconv.rs:989:32
[00:13:02]     |
[00:13:02] 989 |         bound_trait_refs.push((principal, trait_bounds[0].span));
[00:13:02]     |                                ^^^^^^^^^ expected tuple, found struct `rustc::ty::Binder`
[00:13:02]     |
[00:13:02]     = note: expected type `(rustc::ty::Binder<rustc::ty::TraitRef<'_>>, std::option::Option<std::vec::Vec<syntax_pos::Span>>)`
[00:13:02] 
[00:13:02] 
[00:13:02] error[E0271]: type mismatch resolving `<std::vec::Vec<((rustc::ty::Binder<rustc::ty::TraitRef<'_>>, std::option::Option<std::vec::Vec<syntax_pos::Span>>), syntax_pos::Span)> as std::iter::IntoIterator>::Item == (rustc::ty::Binder<rustc::ty::TraitRef<'_>>, syntax_pos::Span)`
[00:13:02]    --> src/librustc_typeck/astconv.rs:991:31
[00:13:02]     |
[00:13:02] 991 |         let expanded_traits = traits::expand_trait_refs(tcx, bound_trait_refs);
[00:13:02]     |                               ^^^^^^^^^^^^^^^^^^^^^^^^^ expected tuple, found struct `rustc::ty::Binder`
[00:13:02]     |
[00:13:02]     = note: expected type `((rustc::ty::Binder<rustc::ty::TraitRef<'_>>, std::option::Option<std::vec::Vec<syntax_pos::Span>>), syntax_pos::Span)`
[00:13:02]                found type `(rustc::ty::Binder<rustc::ty::TraitRef<'_>>, syntax_pos::Span)`
[00:13:02]     = note: required by `rustc::traits::expand_trait_refs`
[00:13:07] error: aborting due to 2 previous errors
[00:13:07] 
[00:13:07] Some errors occurred: E0271, E0308.
[00:13:07] For more information about an error, try `rustc --explain E0271`.
---
travis_time:end:0e13e78f:start=1543031344065173331,finish=1543031344071112066,duration=5938735
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:25f275f9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:12279638
travis_time:start:12279638
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:002334db
$ dmesg | grep -i kill
