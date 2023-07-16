plain
travis_time:end:15475e48:start=1553829009052570560,finish=1553829089757795571,duration=80705225011
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:05:13]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:05:30]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:05:31]    Compiling synstructure v0.10.1
[00:05:43]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:05:43] error[E0560]: struct `snippet::MultilineAnnotation` has no field named `overlaps_exactly`
[00:05:43]    --> src/librustc_errors/emitter.rs:246:25
[00:05:43]     |
[00:05:43] 246 |                         overlaps_exactly: false,
[00:05:43]     |                         ^^^^^^^^^^^^^^^^ `snippet::MultilineAnnotation` does not have this field
[00:05:43]     |
[00:05:43]     = note: available fields are: `depth`, `line_start`, `line_end`, `start_col`, `end_col` ... and 3 others
[00:05:43] 
[00:05:43] error[E0609]: no field `overlaps_exactly` on type `&mut snippet::MultilineAnnotation`
[00:05:43]    --> src/librustc_errors/emitter.rs:280:23
[00:05:43]     |
[00:05:43] 280 |                     a.overlaps_exactly = true;
[00:05:43] 
[00:05:43] 
[00:05:43] error[E0609]: no field `overlaps_exactly` on type `snippet::MultilineAnnotation`
[00:05:43]    --> src/librustc_errors/emitter.rs:293:21
[00:05:43]     |
[00:05:43] 293 |             if !ann.overlaps_exactly {
[00:05:43]     |
[00:05:43]     |
[00:05:43]     = note: available fields are: `depth`, `line_start`, `line_end`, `start_col`, `end_col` ... and 3 others
[00:05:44] error: aborting due to 3 previous errors
[00:05:44] 
[00:05:44] Some errors occurred: E0560, E0609.
[00:05:44] For more information about an error, try `rustc --explain E0560`.
---
198084 ./obj/build/cache/2019-03-20
157452 ./obj/build/bootstrap/debug/incremental
156508 ./src/llvm-project/clang
142560 ./obj/build/bootstrap/debug/incremental/bootstrap-2vehb6dba088a
142556 ./obj/build/bootstrap/debug/incremental/bootstrap-2vehb6dba088a/s-fasba5dc07-myeotd-h9pt7lk04bnw
108532 ./src/llvm-project/lldb
97596 ./src/llvm-project/clang/test
89976 ./src/llvm-emscripten/test/CodeGen
88068 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
