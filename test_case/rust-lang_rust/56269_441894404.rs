plain
travis_time:end:2319eec0:start=1543289811034372568,finish=1543289864911109366,duration=53876736798
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:13:22]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:14:24] error: unsatisfied lifetime constraints
[00:14:24]    --> src/librustc_mir/hair/pattern/_match.rs:341:9
[00:14:24]     |
[00:14:24] 317 |   impl<'a, 'tcx> MatchCheckCtxt<'a, 'tcx> {
[00:14:24]     |        -- lifetime `'a` defined here
[00:14:24] ...
[00:14:24] 335 |       fn lower_byte_str_pattern<'p>(&mut self, pat: &'p Pattern<'tcx>)
[00:14:24]     |                                 -- lifetime `'p` defined here
[00:14:24] ...
[00:14:24] 341 | /         self.byte_array_map.entry(pat).or_insert_with(|| {
[00:14:24] 342 | |             match pat.kind {
[00:14:24] 343 | |                 box PatternKind::Constant {
[00:14:24] 344 | |                     value: const_val
[00:14:24] 372 | |             }
[00:14:24] 373 | |         }).clone()
[00:14:24] 373 | |         }).clone()
[00:14:24]     | |__________________^ returning this value requires that `'p` must outlive `'a`
[00:14:24] 
[00:14:24] error[E0597]: `wild_patterns_owned` does not live long enough
[00:14:24]      |
[00:14:24]      |
[00:14:24] 1254 |     let wild_patterns: Vec<_> = wild_patterns_owned.iter().collect();
[00:14:24]      |                                 ^^^^^^^^^^^^^^^^^^^ borrowed value does not live long enough
[00:14:24] ...
[00:14:24] 1258 |     match specialize(cx, v, &ctor, &wild_patterns) {
[00:14:24]      |           ---------------------------------------- a temporary with access to the borrow is created here ...
[00:14:24] 1269 | }
[00:14:24]      | -
[00:14:24]      | |
[00:14:24]      | |
[00:14:24]      | `wild_patterns_owned` dropped here while still borrowed
[00:14:24]      | ... and the borrow might be used here, when that temporary is dropped and runs the destructor for type `std::option::Option<smallvec::SmallVec<[&hair::pattern::Pattern<'_>; 2]>>`
[00:14:24]      |
[00:14:24]      = note: The temporary is part of an expression at the end of a block. Consider forcing this temporary to be dropped sooner, before the block's local variables are dropped. For example, you could save the expression's value in a new local variable `x` and then make `x` be the expression at the end of the block.
[00:14:24] 
[00:14:24] error[E0597]: `wild_pattern` does not live long enough
[00:14:24]    --> src/librustc_mir/hair/pattern/check_match.rs:287:56
[00:14:24]     |
[00:14:24] 287 |             let witness = match is_useful(cx, &pats, &[&wild_pattern], ConstructWitness) {
[00:14:24]     |                                                        ^^^^^^^^^^^^^ borrowed value does not live long enough
[00:14:24] 309 |         });
[00:14:24]     |         -
[00:14:24]     |         |
[00:14:24]     |         |
[00:14:24]     |         `wild_pattern` dropped here while still borrowed
[00:14:24]     |         borrow might be used here, when `pats` is dropped and runs the destructor for type `hair::pattern::_match::Matrix<'_, '_>`
[00:14:24]     |
[00:14:24]     = note: values in a scope are dropped in the opposite order they are defined
[00:14:24] 
[00:14:24] error[E0597]: `wild_pattern` does not live long enough
[00:14:24]    --> src/librustc_mir/hair/pattern/check_match.rs:475:35
[00:14:24]     |
[00:14:24] 466 | fn check_exhaustive<'a, 'tcx>(cx: &mut MatchCheckCtxt<'a, 'tcx>,
[00:14:24]     |                     -- lifetime `'a` defined here
[00:14:24] ...
[00:14:24] 475 |     match is_useful(cx, matrix, &[&wild_pattern], ConstructWitness) {
[00:14:24]     |           |                       |
[00:14:24]     |           |                       |
[00:14:24]     |           |                       borrowed value does not live long enough
[00:14:24]     |           argument requires that `wild_pattern` is borrowed for `'a`
[00:14:24] 514 | }
[00:14:24] 514 | }
[00:14:24]     | - `wild_pattern` dropped here while still borrowed
[00:14:31] error: aborting due to 4 previous errors
[00:14:31] 
[00:14:31] For more information about this error, try `rustc --explain E0597`.
[00:14:31] error: Could not compile `rustc_mir`.
