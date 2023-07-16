plain
travis_time:end:0f0a66c2:start=1551189756909295875,finish=1551189764282284323,duration=7372988448
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:09:35]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:10:25] error[E0521]: borrowed data escapes outside of function
[00:10:25]    --> src/librustc/traits/auto_trait.rs:353:25
[00:10:25]     |
[00:10:25] 290 |           &self,
[00:10:25]     |           ----- `self` is declared here, outside of the function body
[00:10:25] 291 |           infcx: &InferCtxt<'b, 'tcx, 'c>,
[00:10:25]     |           ----- `infcx` is a reference that is only valid in the function body
[00:10:25] 353 |                       if !self.evaluate_nested_obligations(
[00:10:25]     |  _________________________^
[00:10:25] 354 | |                         ty,
[00:10:25] 355 | |                         obligations,
[00:10:25] 355 | |                         obligations,
[00:10:25] 356 | |                         &mut user_computed_preds,
[00:10:25] ...   |
[00:10:25] 360 | |                         only_projections,
[00:10:25] 361 | |                     ) {
[00:10:25]     | |_____________________^ `infcx` escapes the function body here
[00:10:25] error: unsatisfied lifetime constraints
[00:10:25]    --> src/librustc/traits/auto_trait.rs:697:24
[00:10:25]     |
[00:10:25]     |
[00:10:25] 251 | impl<'a, 'tcx> AutoTraitFinder<'a, 'tcx> {
[00:10:25]     |          ---- lifetime `'tcx` defined here
[00:10:25] 660 |         'cx,
[00:10:25] 660 |         'cx,
[00:10:25]     |         --- lifetime `'cx` defined here
[00:10:25] ...
[00:10:25] 697 |                     if self.is_param_no_infer(p.skip_binder().trait_ref.substs)
[00:10:25]     |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'cx` must outlive `'tcx`
[00:10:25] error[E0521]: borrowed data escapes outside of function
[00:10:25]    --> src/librustc/traits/auto_trait.rs:697:24
[00:10:25]     |
[00:10:25] 663 |         &self,
[00:10:25] 663 |         &self,
[00:10:25]     |         ----- `self` is declared here, outside of the function body
[00:10:25] ...
[00:10:25] 669 |         select: &mut SelectionContext<'c, 'd, 'cx>,
[00:10:25]     |         ------ `select` is a reference that is only valid in the function body
[00:10:25] ...
[00:10:25] 697 |                     if self.is_param_no_infer(p.skip_binder().trait_ref.substs)
[00:10:25]     |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `select` escapes the function body here
[00:10:33] error: aborting due to 3 previous errors
[00:10:33] 
[00:10:33] For more information about this error, try `rustc --explain E0521`.
[00:10:33] error: Could not compile `rustc`.
---
travis_time:end:0272a990:start=1551190409572648952,finish=1551190409579092367,duration=6443415
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:094ff878
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1ddde9d6
travis_time:start:1ddde9d6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1f0e72b4
$ dmesg | grep -i kill
