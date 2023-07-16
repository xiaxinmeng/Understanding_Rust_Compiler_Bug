plain
2020-02-21T12:03:40.4627422Z ========================== Starting Command Output ===========================
2020-02-21T12:03:40.4630905Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/bcb38c67-087c-413f-91c0-9ee2644c3a2c.sh
2020-02-21T12:03:40.4631115Z 
2020-02-21T12:03:40.4634551Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-21T12:03:40.4653435Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-02-21T12:03:40.4657207Z Task         : Get sources
2020-02-21T12:03:40.4657483Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-21T12:03:40.4657803Z Version      : 1.0.0
2020-02-21T12:03:40.4658297Z Author       : Microsoft
---
2020-02-21T12:03:41.4922609Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-21T12:03:41.4927031Z ##[command]git config gc.auto 0
2020-02-21T12:03:41.4930103Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-21T12:03:41.4932869Z ##[command]git config --get-all http.proxy
2020-02-21T12:03:41.4938957Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69274/merge:refs/remotes/pull/69274/merge
---
2020-02-21T12:09:43.0071004Z     Finished release [optimized] target(s) in 1m 25s
2020-02-21T12:09:43.0149667Z tidy check
2020-02-21T12:09:44.2451364Z * 589 error codes
2020-02-21T12:09:44.2452664Z * highest error code: E0746
2020-02-21T12:09:44.5743396Z Expected a gate test for the feature 'target_feature_11'.
2020-02-21T12:09:44.5744634Z Hint: create a failing test file named 'feature-gate-target_feature_11.rs'
2020-02-21T12:09:44.5745436Z       in the 'ui' test suite, with its failures due to
2020-02-21T12:09:44.5745960Z       missing usage of `#![feature(target_feature_11)]`.
2020-02-21T12:09:44.5746897Z Hint: If you already have such a test and don't want to rename it,
2020-02-21T12:09:44.5747679Z       you can also add a // gate-test-target_feature_11 line to the test file.
2020-02-21T12:09:44.5748225Z tidy error: Found 1 features without a gate test.
2020-02-21T12:09:45.3801915Z some tidy checks failed
2020-02-21T12:09:45.3802146Z Found 487 error codes
2020-02-21T12:09:45.3802337Z Found 0 error codes with no tests
2020-02-21T12:09:45.3802695Z Done!
2020-02-21T12:09:45.3802695Z Done!
2020-02-21T12:09:45.3802786Z 
2020-02-21T12:09:45.3802867Z 
2020-02-21T12:09:45.3804187Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-02-21T12:09:45.3804864Z 
2020-02-21T12:09:45.3804947Z 
2020-02-21T12:09:45.3805191Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-02-21T12:09:45.3805507Z Build completed unsuccessfully in 0:01:34
2020-02-21T12:09:45.3805507Z Build completed unsuccessfully in 0:01:34
2020-02-21T12:09:45.3847645Z == clock drift check ==
2020-02-21T12:09:45.3857462Z   local time: Fri Feb 21 12:09:45 UTC 2020
2020-02-21T12:09:45.6764879Z   network time: Fri, 21 Feb 2020 12:09:45 GMT
2020-02-21T12:09:45.6767972Z == end clock drift check ==
2020-02-21T12:09:47.0528957Z 
2020-02-21T12:09:47.0566489Z ##[error]Bash exited with code '1'.
2020-02-21T12:09:47.0583570Z ##[section]Finishing: Run build
2020-02-21T12:09:47.0628264Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-02-21T12:09:47.0632637Z Task         : Get sources
2020-02-21T12:09:47.0632952Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-21T12:09:47.0633241Z Version      : 1.0.0
2020-02-21T12:09:47.0633458Z Author       : Microsoft
2020-02-21T12:09:47.0633458Z Author       : Microsoft
2020-02-21T12:09:47.0633953Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-21T12:09:47.0634328Z ==============================================================================
2020-02-21T12:09:47.3684352Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-21T12:09:47.3731065Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-02-21T12:09:47.3819974Z Cleaning up task key
2020-02-21T12:09:47.3821380Z Start cleaning up orphan processes.
2020-02-21T12:09:47.3987819Z Terminate orphan process: pid (4317) (python)
2020-02-21T12:09:47.4134502Z ##[section]Finishing: Finalize Job
