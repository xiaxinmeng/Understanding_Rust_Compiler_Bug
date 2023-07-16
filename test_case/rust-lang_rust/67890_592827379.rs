plain
2020-02-29T02:27:55.2488052Z ========================== Starting Command Output ===========================
2020-02-29T02:27:55.2491874Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c23767f7-2592-46f0-b476-59446e4cebb6.sh
2020-02-29T02:27:55.2492230Z 
2020-02-29T02:27:55.2496346Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-29T02:27:55.2516419Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67890/merge to s
2020-02-29T02:27:55.2519630Z Task         : Get sources
2020-02-29T02:27:55.2519906Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-29T02:27:55.2520245Z Version      : 1.0.0
2020-02-29T02:27:55.2520429Z Author       : Microsoft
---
2020-02-29T02:27:56.7382547Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-29T02:27:56.7389279Z ##[command]git config gc.auto 0
2020-02-29T02:27:56.7392846Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-29T02:27:56.7396124Z ##[command]git config --get-all http.proxy
2020-02-29T02:27:56.7403749Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67890/merge:refs/remotes/pull/67890/merge
---
2020-02-29T02:34:56.0440596Z     Finished release [optimized] target(s) in 1m 34s
2020-02-29T02:34:56.0544339Z tidy check
2020-02-29T02:34:57.4762949Z * 590 error codes
2020-02-29T02:34:57.4763808Z * highest error code: E0747
2020-02-29T02:34:57.8796569Z Expected a gate test for the feature 'lazy_normalization_consts'.
2020-02-29T02:34:57.8797219Z Hint: create a failing test file named 'feature-gate-lazy_normalization_consts.rs'
2020-02-29T02:34:57.8797786Z       in the 'ui' test suite, with its failures due to
2020-02-29T02:34:57.8798132Z       missing usage of `#![feature(lazy_normalization_consts)]`.
2020-02-29T02:34:57.8798652Z Hint: If you already have such a test and don't want to rename it,
2020-02-29T02:34:57.8800110Z       you can also add a // gate-test-lazy_normalization_consts line to the test file.
2020-02-29T02:34:57.8800540Z tidy error: Found 1 features without a gate test.
2020-02-29T02:34:58.8583821Z Found 488 error codes
2020-02-29T02:34:58.8600112Z Found 0 error codes with no tests
2020-02-29T02:34:58.8600727Z Done!
2020-02-29T02:34:58.8601244Z some tidy checks failed
2020-02-29T02:34:58.8601244Z some tidy checks failed
2020-02-29T02:34:58.8601526Z 
2020-02-29T02:34:58.8601751Z 
2020-02-29T02:34:58.8603519Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-02-29T02:34:58.8605002Z 
2020-02-29T02:34:58.8605544Z 
2020-02-29T02:34:58.8611469Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-02-29T02:34:58.8612196Z Build completed unsuccessfully in 0:01:46
2020-02-29T02:34:58.8612196Z Build completed unsuccessfully in 0:01:46
2020-02-29T02:34:58.8645370Z == clock drift check ==
2020-02-29T02:34:58.8675574Z   local time: Sat Feb 29 02:34:58 UTC 2020
2020-02-29T02:34:59.1553922Z   network time: Sat, 29 Feb 2020 02:34:59 GMT
2020-02-29T02:34:59.1557889Z == end clock drift check ==
2020-02-29T02:34:59.9436855Z 
2020-02-29T02:34:59.9528510Z ##[error]Bash exited with code '1'.
2020-02-29T02:34:59.9544024Z ##[section]Finishing: Run build
2020-02-29T02:34:59.9591647Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67890/merge to s
2020-02-29T02:34:59.9596907Z Task         : Get sources
2020-02-29T02:34:59.9597261Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-29T02:34:59.9597739Z Version      : 1.0.0
2020-02-29T02:34:59.9597966Z Author       : Microsoft
2020-02-29T02:34:59.9597966Z Author       : Microsoft
2020-02-29T02:34:59.9598324Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-29T02:34:59.9598725Z ==============================================================================
2020-02-29T02:35:00.3129093Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-29T02:35:00.3174690Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67890/merge to s
2020-02-29T02:35:00.3264502Z Cleaning up task key
2020-02-29T02:35:00.3265766Z Start cleaning up orphan processes.
2020-02-29T02:35:00.3445671Z Terminate orphan process: pid (4046) (python)
2020-02-29T02:35:00.3613716Z ##[section]Finishing: Finalize Job
