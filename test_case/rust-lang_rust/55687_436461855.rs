plain
travis_time:end:27cc017a:start=1541549967291874173,finish=1541549969616105461,duration=2324231288
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:19:25]    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
[00:19:28] error: method is never used: `trait_ref_to_existential`
[00:19:28]    --> librustc_typeck/astconv.rs:962:5
[00:19:28]     |
[00:19:28] 962 | /     fn trait_ref_to_existential(&self, trait_ref: ty::TraitRef<'tcx>)
[00:19:28] 963 | |                                 -> ty::ExistentialTraitRef<'tcx> {
[00:19:28] 964 | |         assert_eq!(trait_ref.self_ty().sty, TRAIT_OBJECT_DUMMY_SELF);
[00:19:28] 965 | |         ty::ExistentialTraitRef::erase_self_ty(self.tcx(), trait_ref)
[00:19:28]     | |_____^
[00:19:28]     |
[00:19:28]     = note: `-D dead-code` implied by `-D warnings`
[00:19:28] 
