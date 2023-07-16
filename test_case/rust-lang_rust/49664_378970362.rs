plain
[00:01:29] configure: rust.quiet-tests     := True
---
]       in the 'ui' test suite, with its failures due to
[00:05:07]       missing usage of #![feature(aarch64_target_feature)].
[00:05:07] Hint: If you already have such a test and don't want to rename it,
[00:05:07]       you can also add a // gate-test-aarch64_target_feature line to the test file.
[00:05:07] Expected a gate test for the feature 'mmx_target_feature'.
[00:05:07] Hint: create a failing test file named 'feature-gate-mmx_target_feature.rs'
[00:05:07]       in the 'ui' test suite, with its failures due to
[00:05:07]       missing usage of #![feature(mmx_target_feature)].
[00:05:07] Hint: If you already have such a test and don't want to rename it,
[00:05:07]       you can also add a // gate-test-mmx_target_feature line to the test file.
[00:05:07] Expected a gate test for the feature 'powerpc_target_feature'.
[00:05:07] Hint: create a failing test file named 'feature-gate-powerpc_target_feature.rs'
[00:05:07]       in the 'ui' test suite, with its failures due to
[00:05:07]       missing usage of #![feature(powerpc_target_feature)].
[00:05:07] Hint: If you already have such a test and don't want to rename it,
[00:05:07]       you can also add a // gate-test-powerpc_target_feature line to the test file.
[00:05:07] Expected a gate test for the feature 'sse4a_target_feature'.
[00:05:07] Hint: create a failing test file named 'feature-gate-sse4a_target_feature.rs'
[00:05:07]       in the 'ui' test suite, with its failures due to
[00:05:07]       missing usage of #![feature(sse4a_target_feature)].
[00:05:07] Hint: If you already have such a test and don't want to rename it,
[00:05:07]       you can also add a // gate-test-sse4a_target_feature line to the test file.
[00:05:07] Expected a gate test for the feature 'avx512_target_feature'.
[00:05:07] Hint: create a failing test file named 'feature-gate-avx512_target_feature.rs'
[00:05:07]       in the 'ui' test suite, with its failures due to
[00:05:07]       missing usage of #![feature(avx512_target_feature)].
[00:05:07] Hint: If you already have such a test and don't want to rename it,
[00:05:07]       you can also add a // gate-test-avx512_target_feature line to the test file.
[00:05:07] Expected a gate test for the feature 'tbm_target_feature'.
[00:05:07] Hint: create a failing test file named 'feature-gate-tbm_target_feature.rs'
[00:05:07]       in the 'ui' test suite, with its failures due to
[00:05:07]       missing usage of #![feature(tbm_target_feature)].
[00:05:07] Hint: If you already have such a test and don't want to rename it,
[00:05:07]       you can also add a // gate-test-tbm_target_feature line to the test file.
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:3995dc75:start=1522941035763776228,finish=1522941035769506889,duration=5730661
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:1833ec00
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:1833ec00:start=1522941035774367335,finish=1522941035779994962,duration=5627627
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0fb35062
$ dmesg | grep -i kill
[   10.571310] init: failsafe main process (1093) killed by TERM signal
