plain
travis_time:end:00a1e296:start=1554378817381314523,finish=1554378818338907445,duration=957592922
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:06:43]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:54]    Compiling synstructure v0.10.1
[00:07:14]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:08:06]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:08:12] error[E0425]: cannot find value `LOCAL_CRATE` in this scope
[00:08:12]     --> src/librustc/ty/query/plumbing.rs:1197:13
[00:08:12] 1197 |             LOCAL_CRATE,
[00:08:12]      |             ^^^^^^^^^^^ not found in this scope
[00:08:12] help: possible candidate is found in another module, you can import it into scope
[00:08:12]      |
[00:08:12]      |
[00:08:12] 5    | use crate::hir::def_id::LOCAL_CRATE;
[00:08:12]      |
[00:08:12] 
[00:08:12] error[E0425]: cannot find value `LOCAL_CRATE` in this scope
[00:08:12]     --> src/librustc/ty/query/plumbing.rs:1199:55
[00:08:12]      |
[00:08:12] 1199 |             DepNode::new(tcx, DepConstructor::hir_map(LOCAL_CRATE)),
[00:08:12] help: possible candidate is found in another module, you can import it into scope
[00:08:12]      |
[00:08:12] 5    | use crate::hir::def_id::LOCAL_CRATE;
[00:08:12]      |
