plain
travis_time:end:26a2f0f0:start=1544107249429051519,finish=1544107306844819004,duration=57415767485
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:20:06]    Compiling rustc-rayon v0.1.1
[00:20:09]    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[00:20:12]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:20:13]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:20:14] error: use of deprecated item 'core::str::<impl str>::trim_left_matches': superseded by `trim_start_matches`
[00:20:14]   --> src/libsyntax_pos/symbol.rs:58:49
[00:20:14]    |
[00:20:14] 58 |         Ident::new(Symbol::intern(self.as_str().trim_left_matches('\'')), self.span)
[00:20:14]    |
[00:20:14]    = note: `-D deprecated` implied by `-D warnings`
[00:20:14] 
[00:20:14] error: aborting due to previous error
---
187096 ./obj/build/cache
187092 ./obj/build/cache/2018-12-05
159656 ./obj/build/bootstrap/debug/incremental
153280 ./src/tools/clang
143564 ./obj/build/bootstrap/debug/incremental/bootstrap-gg4wocctfx8q
143560 ./obj/build/bootstrap/debug/incremental/bootstrap-gg4wocctfx8q/s-f7c95n485k-v2rsjd-1jpjqomavnfay
134904 ./.git/modules/src
115356 ./src/llvm/test/CodeGen
111164 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
107420 ./src/tools/lldb
