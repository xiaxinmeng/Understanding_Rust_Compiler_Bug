plain
2019-10-22T17:58:21.8750390Z [RUSTC-TIMING] rustfix test:false 5.039
2019-10-22T17:58:22.1847560Z error[E0433]: failed to resolve: could not find `OutputFormat` in `test`
2019-10-22T17:58:22.1854678Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.24/src/lib.rs:116:41
2019-10-22T17:58:22.1860440Z     |
2019-10-22T17:58:22.1866719Z 116 |         format: if config.quiet { test::OutputFormat::Terse } else { test::OutputFormat::Pretty },
2019-10-22T17:58:22.1873491Z     |                                         ^^^^^^^^^^^^ could not find `OutputFormat` in `test`
2019-10-22T17:58:22.1884953Z error[E0433]: failed to resolve: could not find `OutputFormat` in `test`
2019-10-22T17:58:22.1922327Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.24/src/lib.rs:116:76
2019-10-22T17:58:22.1922989Z     |
2019-10-22T17:58:22.1922989Z     |
2019-10-22T17:58:22.1923923Z 116 |         format: if config.quiet { test::OutputFormat::Terse } else { test::OutputFormat::Pretty },
2019-10-22T17:58:22.1924462Z     |                                                                            ^^^^^^^^^^^^ could not find `OutputFormat` in `test`
2019-10-22T17:58:22.1931215Z error[E0425]: cannot find function `run_tests_console` in crate `test`
2019-10-22T17:58:22.1937704Z   --> /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.24/src/lib.rs:99:21
2019-10-22T17:58:22.2014606Z    |
2019-10-22T17:58:22.2014606Z    |
2019-10-22T17:58:22.2017981Z 99 |     let res = test::run_tests_console(&opts, tests.into_iter().collect());
2019-10-22T17:58:22.2019752Z 
2019-10-22T17:58:22.2021439Z error[E0603]: enum `ColorConfig` is private
2019-10-22T17:58:22.2022030Z   --> /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.24/src/common.rs:20:11
2019-10-22T17:58:22.2022550Z    |
2019-10-22T17:58:22.2022550Z    |
2019-10-22T17:58:22.2023166Z 20 | use test::ColorConfig;
2019-10-22T17:58:22.2023542Z    |           ^^^^^^^^^^^
2019-10-22T17:58:22.2023602Z 
2019-10-22T17:58:22.2023866Z error[E0603]: struct `TestOpts` is private
2019-10-22T17:58:22.2024199Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.24/src/lib.rs:109:44
2019-10-22T17:58:22.2024458Z     |
2019-10-22T17:58:22.2024792Z 109 | pub fn test_opts(config: &Config) -> test::TestOpts {
2019-10-22T17:58:22.2025198Z 
2019-10-22T17:58:22.2025771Z error[E0603]: struct `TestOpts` is private
2019-10-22T17:58:22.2026110Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.24/src/lib.rs:110:11
2019-10-22T17:58:22.2026358Z     |
2019-10-22T17:58:22.2026358Z     |
2019-10-22T17:58:22.2026651Z 110 |     test::TestOpts {
2019-10-22T17:58:22.2026933Z     |           ^^^^^^^^
2019-10-22T17:58:22.2027006Z 
2019-10-22T17:58:22.2027254Z error[E0603]: enum `RunIgnored` is private
2019-10-22T17:58:22.2027612Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.24/src/lib.rs:115:52
2019-10-22T17:58:22.2027871Z     |
2019-10-22T17:58:22.2028743Z 115 |         run_ignored: if config.run_ignored { test::RunIgnored::Yes } else { test::RunIgnored::No },
2019-10-22T17:58:22.2029259Z 
2019-10-22T17:58:22.2029554Z error[E0603]: enum `RunIgnored` is private
2019-10-22T17:58:22.2029925Z    --> /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.24/src/lib.rs:115:83
2019-10-22T17:58:22.2030231Z     |
2019-10-22T17:58:22.2030231Z     |
2019-10-22T17:58:22.2030652Z 115 |         run_ignored: if config.run_ignored { test::RunIgnored::Yes } else { test::RunIgnored::No },
2019-10-22T17:58:22.2031219Z 
2019-10-22T17:58:23.2771842Z error: aborting due to 8 previous errors
2019-10-22T17:58:23.2776024Z 
2019-10-22T17:58:23.2783343Z Some errors have detailed explanations: E0425, E0433, E0603.
---
2019-10-22T18:46:38.3023526Z Verifying status of rustfmt...
2019-10-22T18:46:38.3036788Z Verifying status of clippy-driver...
2019-10-22T18:46:38.3052000Z This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
2019-10-22T18:46:38.3063293Z 
2019-10-22T18:46:38.3064078Z ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
2019-10-22T18:46:38.3064313Z 
2019-10-22T18:46:38.3064974Z If you do intend to update 'clippy-driver', please check the error messages above and
2019-10-22T18:46:38.3065241Z commit another update.
2019-10-22T18:46:38.3065362Z 
2019-10-22T18:46:38.3066409Z If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
2019-10-22T18:46:38.3066926Z change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
2019-10-22T18:46:38.3067008Z proper steps.
2019-10-22T18:46:38.3092113Z   local time: Tue Oct 22 18:46:38 UTC 2019
2019-10-22T18:46:38.5968235Z   network time: Tue, 22 Oct 2019 18:46:38 GMT
2019-10-22T18:46:38.5968462Z == end clock drift check ==
2019-10-22T18:46:39.2132471Z 
2019-10-22T18:46:39.2132471Z 
2019-10-22T18:46:39.2247503Z ##[error]Bash exited with code '3'.
2019-10-22T18:46:39.2291083Z ##[section]Starting: Upload CPU usage statistics
2019-10-22T18:46:39.2303895Z ==============================================================================
2019-10-22T18:46:39.2304003Z Task         : Bash
2019-10-22T18:46:39.2304101Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-22T18:46:39.3876028Z Script contents:
2019-10-22T18:46:39.3876613Z aws s3 cp --acl public-read cpu-usage.csv s3://$DEPLOY_BUCKET/rustc-builds/$BUILD_SOURCEVERSION/cpu-$CI_JOB_NAME.csv
2019-10-22T18:46:39.3895206Z ========================== Starting Command Output ===========================
2019-10-22T18:46:39.3916405Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8f9f2656-7cb7-43e0-a3f2-cec708f90d16.sh
2019-10-22T18:47:43.0098960Z upload failed: ./cpu-usage.csv to s3://rust-lang-ci2/rustc-builds/68cfd58d9bb322ecc1abaddbfe5c8589bca16329/cpu-x86_64-gnu-tools.csv Could not connect to the endpoint URL: "https://rust-lang-ci2.s3.amazonaws.com/rustc-builds/68cfd58d9bb322ecc1abaddbfe5c8589bca16329/cpu-x86_64-gnu-tools.csv"
2019-10-22T18:47:43.1041465Z 
2019-10-22T18:47:43.1077655Z ##[error]Bash exited with code '1'.
2019-10-22T18:47:43.1112378Z ##[section]Starting: Checkout
2019-10-22T18:47:43.1114126Z ==============================================================================
2019-10-22T18:47:43.1114228Z Task         : Get sources
2019-10-22T18:47:43.1114306Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
