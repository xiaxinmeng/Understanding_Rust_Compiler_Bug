plain
travis_time:end:14ce26ba:start=1544532111472385763,finish=1544532112534669366,duration=1062283603
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:05:28]    Compiling parking_lot v0.6.4
[00:05:28]    Compiling rustc-rayon v0.1.1
[00:05:31]    Compiling tempfile v3.0.5
[00:05:32]    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[00:05:37] error[E0560]: struct `spec::Target` has no field named `features`
[00:05:37]   --> src/librustc_target/spec/x86_64_fortanix_unknown_sgx.rs:69:9
[00:05:37]    |
[00:05:37] 69 |         features: "rdrnd,rdseed".into(),
[00:05:37]    |         ^^^^^^^^ `spec::Target` does not have this field
[00:05:37]    |
[00:05:37]    = note: available fields are: `llvm_target`, `target_endian`, `target_pointer_width`, `target_c_int_width`, `target_os` ... and 6 others
[00:05:37] error: aborting due to previous error
[00:05:37] 
[00:05:37] For more information about this error, try `rustc --explain E0560`.
[00:05:37] error: Could not compile `rustc_target`.
---
153288 ./src/tools/clang
150704 ./obj/build/bootstrap/debug/incremental
149568 ./.git
135104 ./obj/build/bootstrap/debug/incremental/bootstrap-2pgjvb3usndhe
135100 ./obj/build/bootstrap/debug/incremental/bootstrap-2pgjvb3usndhe/s-f7hob0oiws-ukbbk3-e1ql6nnu93qh
134556 ./.git/modules/src
115344 ./src/llvm/test/CodeGen
107888 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
107416 ./src/tools/lldb
