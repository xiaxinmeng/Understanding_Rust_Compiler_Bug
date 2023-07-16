plain
travis_time:end:2062c72c:start=1554308928291343468,finish=1554308930578655847,duration=2287312379
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:33:41]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:41:54]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:41:54]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:41:54]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:42:31] error: usage of `ty::TyKind::<kind>`
[00:42:31]    --> src/librustc_mir/borrow_check/nll/type_check/mod.rs:749:21
[00:42:31]     |
[00:42:31] 749 |                     ty::TyKind::Adt(adt_def, substs) => (&adt_def.variants[variant_index], substs),
[00:42:31]     |                     ^^^^^^^^^^ help: try using ty::<kind> directly: `ty`
[00:42:31] note: lint level defined here
[00:42:31]    --> src/librustc_mir/lib.rs:31:31
[00:42:31]     |
[00:42:31]     |
[00:42:31] 31  | #![cfg_attr(not(stage0), deny(internal))]
[00:42:31]     |                               ^^^^^^^^
[00:42:31]     = note: #[deny(usage_of_ty_tykind)] implied by #[deny(internal)]
[00:42:32] error: aborting due to previous error
[00:42:32] 
[00:42:32] error: Could not compile `rustc_mir`.
[00:42:32] warning: build failed, waiting for other jobs to finish...
