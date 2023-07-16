plain
travis_time:end:0287f26a:start=1544146933139304632,finish=1544146936018866977,duration=2879562345
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:05:33]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:05:34]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:05:38]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:03]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:05] error[E0432]: unresolved import `ty::subst::Subts`
[00:07:05]   --> src/librustc/traits/util.rs:16:30
[00:07:05]    |
[00:07:05] 16 | use ty::subst::{Kind, Subst, Subts};
[00:07:05]    |                              ^^^^^ no `Subts` in `ty::subst`. Did you mean to use `Substs`?
[00:07:06] error[E0412]: cannot find type `Substs` in this scope
[00:07:06]    --> src/librustc/traits/util.rs:371:63
[00:07:06]     |
[00:07:06]     |
[00:07:06] 371 |                                                 impl_substs: &Substs<'tcx>)
[00:07:06]     |                                                               ^^^^^^ did you mean `Subst`?
[00:07:06]     |
[00:07:06] 11  | use ty::subst::Substs;
[00:07:06]     |
[00:07:06] 
[00:07:06] 
[00:07:08] error: unused import: `Subts`
[00:07:08]   --> src/librustc/traits/util.rs:16:30
[00:07:08]    |
[00:07:08] 16 | use ty::subst::{Kind, Subst, Subts};
[00:07:08]    |
[00:07:08]    = note: `-D unused-imports` implied by `-D warnings`
[00:07:08] 
=1544147403124499596,finish=1544147403129437222,duration=4937626
