plain
travis_time:end:07014da4:start=1543721947805530572,finish=1543722001731649025,duration=53926118453
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:05:13]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:30]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:11:57]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:11:57]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:12:07] error[E0063]: missing field `two_phase` in initializer of `rustc::mir::StatementKind<'_>`
[00:12:07]     |
[00:12:07]     |
[00:12:07] 232 |                 kind: StatementKind::Retag { fn_entry: true, place: dropee_ptr.clone() },
[00:12:07]     |                       ^^^^^^^^^^^^^^^^^^^^ missing `two_phase`
[00:12:11] error: aborting due to previous error
[00:12:11] 
[00:12:11] For more information about this error, try `rustc --explain E0063`.
[00:12:11] error: Could not compile `rustc_mir`.
---
184276 ./obj/build/cache/2018-10-30
153280 ./src/tools/clang
150332 ./obj/build/bootstrap/debug/incremental
134740 ./obj/build/bootstrap/debug/incremental/bootstrap-1plb86h2refwc
134736 ./obj/build/bootstrap/debug/incremental/bootstrap-1plb86h2refwc/s-f77c5egytw-1wv8owa-4qyniat88vxc
134212 ./.git/modules/src
130292 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
115356 ./src/llvm/test/CodeGen
107888 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
