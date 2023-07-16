plain
2020-02-10T21:43:51.8071595Z ========================== Starting Command Output ===========================
2020-02-10T21:43:51.8073826Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/545eab5c-e94e-4a02-85a3-60f77d33b5eb.sh
2020-02-10T21:43:51.8073860Z 
2020-02-10T21:43:51.8076671Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-10T21:43:51.8081537Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68823/merge to s
2020-02-10T21:43:51.8082971Z Task         : Get sources
2020-02-10T21:43:51.8083006Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-10T21:43:51.8083030Z Version      : 1.0.0
2020-02-10T21:43:51.8083054Z Author       : Microsoft
---
2020-02-10T21:43:52.6551927Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-10T21:43:52.6644230Z ##[command]git config gc.auto 0
2020-02-10T21:43:52.6707518Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-10T21:43:52.6752414Z ##[command]git config --get-all http.proxy
2020-02-10T21:43:52.6878997Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68823/merge:refs/remotes/pull/68823/merge
---
2020-02-10T21:49:34.5698436Z * 589 error codes
2020-02-10T21:49:34.5698532Z * highest error code: E0746
2020-02-10T21:49:34.9131955Z * 280 features
2020-02-10T21:49:35.4691048Z crate `rustc-ap-syntax` is duplicated in `Cargo.lock`
2020-02-10T21:49:35.4691572Z   * rustc-ap-syntax 642.0.0 (registry+https://github.com/rust-lang/crates.io-index)
2020-02-10T21:49:35.4691829Z   * rustc-ap-syntax 610.0.0 (registry+https://github.com/rust-lang/crates.io-index)
2020-02-10T21:49:35.6986949Z Found 487 error codes
2020-02-10T21:49:35.6987062Z Found 0 error codes with no tests
2020-02-10T21:49:35.6987138Z Done!
2020-02-10T21:49:35.6987170Z some tidy checks failed
2020-02-10T21:49:35.6987170Z some tidy checks failed
2020-02-10T21:49:35.6987193Z 
2020-02-10T21:49:35.6987213Z 
2020-02-10T21:49:35.6988150Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-02-10T21:49:35.6988267Z 
2020-02-10T21:49:35.6988306Z 
2020-02-10T21:49:35.6988345Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-02-10T21:49:35.6988384Z Build completed unsuccessfully in 0:01:32
2020-02-10T21:49:35.6988384Z Build completed unsuccessfully in 0:01:32
2020-02-10T21:49:35.7030909Z == clock drift check ==
2020-02-10T21:49:35.7041393Z   local time: Mon Feb 10 21:49:35 UTC 2020
2020-02-10T21:49:35.9942576Z   network time: Mon, 10 Feb 2020 21:49:35 GMT
2020-02-10T21:49:35.9942651Z == end clock drift check ==
2020-02-10T21:49:36.7268376Z 
2020-02-10T21:49:36.7355952Z ##[error]Bash exited with code '1'.
2020-02-10T21:49:36.7377972Z ##[section]Finishing: Run build
2020-02-10T21:49:36.7394660Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68823/merge to s
2020-02-10T21:49:36.7397140Z Task         : Get sources
2020-02-10T21:49:36.7397180Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-10T21:49:36.7397400Z Version      : 1.0.0
2020-02-10T21:49:36.7397433Z Author       : Microsoft
2020-02-10T21:49:36.7397433Z Author       : Microsoft
2020-02-10T21:49:36.7397472Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-10T21:49:36.7397712Z ==============================================================================
2020-02-10T21:49:37.1285231Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-10T21:49:37.1325077Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68823/merge to s
2020-02-10T21:49:37.1417072Z Cleaning up task key
2020-02-10T21:49:37.1417732Z Start cleaning up orphan processes.
2020-02-10T21:49:37.1519615Z Terminate orphan process: pid (3792) (python)
2020-02-10T21:49:37.1681156Z ##[section]Finishing: Finalize Job
