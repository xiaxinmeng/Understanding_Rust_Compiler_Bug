plain
2020-01-25T20:55:33.3398382Z ========================== Starting Command Output ===========================
2020-01-25T20:55:33.3413247Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/994373e8-2fe4-4862-a1a9-67a94f80c887.sh
2020-01-25T20:55:34.1479173Z 
2020-01-25T20:55:34.1548115Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-25T20:55:34.1553823Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68544/merge to s
2020-01-25T20:55:34.1555444Z Task         : Get sources
2020-01-25T20:55:34.1555523Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-25T20:55:34.1555559Z Version      : 1.0.0
2020-01-25T20:55:34.1555596Z Author       : Microsoft
---
2020-01-25T20:55:39.4702219Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-25T20:55:39.5059991Z ##[command]git config gc.auto 0
2020-01-25T20:55:39.5137021Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-25T20:55:39.5197888Z ##[command]git config --get-all http.proxy
2020-01-25T20:55:39.5329337Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68544/merge:refs/remotes/pull/68544/merge
---
2020-01-25T21:02:24.2883489Z 
2020-01-25T21:02:24.2937488Z error[E0432]: unresolved import `removed::REMOVED_FEATURES`
2020-01-25T21:02:24.2938118Z    --> src/librustc_feature/lib.rs:137:19
2020-01-25T21:02:24.2938565Z     |
2020-01-25T21:02:24.2939042Z 137 | pub use removed::{REMOVED_FEATURES, STABLE_REMOVED_FEATURES};
2020-01-25T21:02:24.2939589Z     |                   ^^^^^^^^^^^^^^^^ no `REMOVED_FEATURES` in `removed`
2020-01-25T21:02:24.4407429Z error: aborting due to 2 previous errors
2020-01-25T21:02:24.4408076Z 
2020-01-25T21:02:24.4414831Z For more information about this error, try `rustc --explain E0432`.
2020-01-25T21:02:24.4461099Z error: could not compile `rustc_feature`.
2020-01-25T21:02:24.4461099Z error: could not compile `rustc_feature`.
2020-01-25T21:02:24.4472430Z warning: build failed, waiting for other jobs to finish...
2020-01-25T21:02:25.3086775Z error: build failed
2020-01-25T21:02:25.3106171Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-25T21:02:25.3121118Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-01-25T21:02:25.3121475Z Build completed unsuccessfully in 0:03:33
2020-01-25T21:02:25.3164788Z == clock drift check ==
2020-01-25T21:02:25.3191697Z   local time: Sat Jan 25 21:02:25 UTC 2020
2020-01-25T21:02:25.3191697Z   local time: Sat Jan 25 21:02:25 UTC 2020
2020-01-25T21:02:25.6011935Z   network time: Sat, 25 Jan 2020 21:02:25 GMT
2020-01-25T21:02:25.6017477Z == end clock drift check ==
2020-01-25T21:02:26.4227559Z 
2020-01-25T21:02:26.4313907Z ##[error]Bash exited with code '1'.
2020-01-25T21:02:26.4325462Z ##[section]Finishing: Run build
2020-01-25T21:02:26.4339226Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68544/merge to s
2020-01-25T21:02:26.4341003Z Task         : Get sources
2020-01-25T21:02:26.4341051Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-25T21:02:26.4341112Z Version      : 1.0.0
2020-01-25T21:02:26.4341156Z Author       : Microsoft
2020-01-25T21:02:26.4341156Z Author       : Microsoft
2020-01-25T21:02:26.4341202Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-25T21:02:26.4341267Z ==============================================================================
2020-01-25T21:02:26.8128590Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-25T21:02:26.8198868Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68544/merge to s
2020-01-25T21:02:26.8302224Z Cleaning up task key
2020-01-25T21:02:26.8302980Z Start cleaning up orphan processes.
2020-01-25T21:02:26.8398349Z Terminate orphan process: pid (3906) (python)
2020-01-25T21:02:26.8567346Z ##[section]Finishing: Finalize Job
