plain
travis_time:end:2100fa77:start=1548761976823082028,finish=1548762053487803809,duration=76664721781
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-tools
---
[01:49:37] test actions::hover::test::test_noindent ... ok
[01:49:37] test actions::hover::test::test_process_docs_racer_returns_extra_slashes ... ok
[01:49:37] test actions::hover::test::test_process_docs_rust_blocks ... ok
[01:49:37] test actions::requests::test::test_sort_deglob_str ... ok
[01:49:37] test actions::notifications::test::learn_client_use_change_watched ... ok
[01:49:37] test actions::hover::test::test_format_object ... ok
[01:49:37] test actions::hover::test::test_format_object ... ok
[01:49:37] test actions::test::did_save_relevant_files ... ok
[01:49:37] test build::dont_auto_tune_build_wait_configured ... ok
[01:49:37] test actions::hover::test::test_format_method ... ok
[01:49:37] test build::auto_tune_build_wait_no_config ... ok
[01:49:37] test build::cargo::test::test_dedup_flags ... ok
---
[01:49:50] 
[01:49:50] running 4 tests
[01:49:50] test test_tooltip_std ... ignored
[01:49:50] test test_tooltip_std_racer ... ignored
[01:50:50] test test_tooltip ... test test_tooltip has been running for over 60 seconds
[01:50:50] test test_tooltip_racer ... test test_tooltip_racer has been running for over 60 seconds
No output has been received in the last 30m0s, this potentially indicates a stalled build or something wrong with the build itself.
Check the details on how to adjust your build configuration on: https://docs.travis-ci.com/user/common-build-problems/#Build-times-out-because-no-output-was-received
The build has been terminated
