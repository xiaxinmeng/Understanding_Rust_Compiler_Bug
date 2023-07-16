plain
travis_time:end:1b7891b0:start=1543521223457964898,finish=1543521288000296381,duration=64542331483
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:05:01]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:05:06]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:24]    Compiling proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:06:33]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:06:53] error[E0026]: variant `infer::type_variable::TypeVariableValue::Unknown` does not have a field named `_universe`
[00:06:53]    --> src/librustc/infer/nll_relate/mod.rs:763:54
[00:06:53]     |
[00:06:53] 763 |                         TypeVariableValue::Unknown { _universe } => {
[00:06:53]     |                                                      |
[00:06:53]     |                                                      |
[00:06:53]     |                                                      variant `infer::type_variable::TypeVariableValue::Unknown` does not have this field
[00:06:53]     |                                                      help: did you mean: `universe`
travis_time:end:15ff275a:start=1543521296609365783,finish=1543521721649467152,duration=425040101369
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:000ecf04
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
