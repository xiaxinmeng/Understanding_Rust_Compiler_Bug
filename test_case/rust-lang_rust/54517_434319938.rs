plain
travis_time:end:0567a1a0:start=1540908825520342378,finish=1540908827859993867,duration=2339651489
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:06:31]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:39] error[E0433]: failed to resolve. Use of undeclared type or module `StableFilemapId`
[00:06:39]    --> libsyntax/source_map.rs:113:9
[00:06:39]     |
[00:06:39] 113 |         StableFilemapId::new_from_pieces(&source_file.name,
[00:06:39]     |         ^^^^^^^^^^^^^^^ Use of undeclared type or module `StableFilemapId`
[00:06:39] error[E0433]: failed to resolve. Use of undeclared type or module `StableFilemapId`
[00:06:39]    --> libsyntax/source_map.rs:239:23
[00:06:39]     |
[00:06:39]     |
[00:06:39] 239 |         let file_id = StableFilemapId::new_from_pieces(&filename,
[00:06:39]     |                       ^^^^^^^^^^^^^^^ Use of undeclared type or module `StableFilemapId`
[00:06:39] 
[00:06:39] error[E0412]: cannot find type `StableFilemapId` in this scope
[00:06:39]    --> libsyntax/source_map.rs:120:65
[00:06:39]     |
[00:06:39] 120 |                            unmapped_path: Option<&FileName>) -> StableFilemapId {
[00:06:39] 
travis_time:end:2324ffd7:start=1540908837515088323,finish=1540909242604311853,duration=405089223530

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
