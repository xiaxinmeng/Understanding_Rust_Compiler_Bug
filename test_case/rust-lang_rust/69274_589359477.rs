plain
2020-02-20T21:43:59.4615802Z ========================== Starting Command Output ===========================
2020-02-20T21:43:59.4618554Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a8ddd576-ce16-4088-be5d-28fa5406a67d.sh
2020-02-20T21:43:59.4618818Z 
2020-02-20T21:43:59.4622794Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-20T21:43:59.4643489Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-02-20T21:43:59.4647083Z Task         : Get sources
2020-02-20T21:43:59.4647380Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-20T21:43:59.4647666Z Version      : 1.0.0
2020-02-20T21:43:59.4647920Z Author       : Microsoft
---
2020-02-20T21:44:00.4586097Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-20T21:44:00.4592550Z ##[command]git config gc.auto 0
2020-02-20T21:44:00.4626475Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-20T21:44:00.4630924Z ##[command]git config --get-all http.proxy
2020-02-20T21:44:00.4638914Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69274/merge:refs/remotes/pull/69274/merge
---
2020-02-20T21:50:30.9165696Z     Finished release [optimized] target(s) in 1m 34s
2020-02-20T21:50:30.9268626Z tidy check
2020-02-20T21:50:32.3681980Z * 589 error codes
2020-02-20T21:50:32.3682215Z * highest error code: E0746
2020-02-20T21:50:32.7541167Z Expected a gate test for the feature 'target_feature_11'.
2020-02-20T21:50:32.7541804Z Hint: create a failing test file named 'feature-gate-target_feature_11.rs'
2020-02-20T21:50:32.7542351Z       in the 'ui' test suite, with its failures due to
2020-02-20T21:50:32.7542710Z       missing usage of `#![feature(target_feature_11)]`.
2020-02-20T21:50:32.7543251Z Hint: If you already have such a test and don't want to rename it,
2020-02-20T21:50:32.7543845Z       you can also add a // gate-test-target_feature_11 line to the test file.
2020-02-20T21:50:32.7544217Z tidy error: Found 1 features without a gate test.
2020-02-20T21:50:33.7094627Z Found 487 error codes
2020-02-20T21:50:33.7095564Z Found 0 error codes with no tests
2020-02-20T21:50:33.7096091Z Done!
2020-02-20T21:50:33.7106201Z some tidy checks failed
2020-02-20T21:50:33.7106201Z some tidy checks failed
2020-02-20T21:50:33.7107601Z 
2020-02-20T21:50:33.7107876Z 
2020-02-20T21:50:33.7109354Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-02-20T21:50:33.7110771Z 
2020-02-20T21:50:33.7111009Z 
2020-02-20T21:50:33.7116435Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-02-20T21:50:33.7116848Z Build completed unsuccessfully in 0:01:46
2020-02-20T21:50:33.7116848Z Build completed unsuccessfully in 0:01:46
2020-02-20T21:50:33.7163837Z == clock drift check ==
2020-02-20T21:50:33.7176573Z   local time: Thu Feb 20 21:50:33 UTC 2020
2020-02-20T21:50:34.0039399Z   network time: Thu, 20 Feb 2020 21:50:34 GMT
2020-02-20T21:50:34.0039862Z == end clock drift check ==
2020-02-20T21:50:34.8202816Z 
2020-02-20T21:50:34.8239688Z ##[error]Bash exited with code '1'.
2020-02-20T21:50:34.8253566Z ##[section]Finishing: Run build
2020-02-20T21:50:34.8301965Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-02-20T21:50:34.8306629Z Task         : Get sources
2020-02-20T21:50:34.8306979Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-20T21:50:34.8308347Z Version      : 1.0.0
2020-02-20T21:50:34.8308611Z Author       : Microsoft
2020-02-20T21:50:34.8308611Z Author       : Microsoft
2020-02-20T21:50:34.8308986Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-20T21:50:34.8309441Z ==============================================================================
2020-02-20T21:50:35.1811719Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-20T21:50:35.1861703Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-02-20T21:50:35.1955896Z Cleaning up task key
2020-02-20T21:50:35.1957502Z Start cleaning up orphan processes.
2020-02-20T21:50:35.2266759Z Terminate orphan process: pid (3778) (python)
2020-02-20T21:50:35.2310826Z ##[section]Finishing: Finalize Job
