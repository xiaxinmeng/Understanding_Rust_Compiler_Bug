plain
2020-01-24T01:55:56.6780967Z ========================== Starting Command Output ===========================
2020-01-24T01:55:56.6783322Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/bd674d30-0b25-4b02-9bf6-08e1b7d022e9.sh
2020-01-24T01:55:56.6783376Z 
2020-01-24T01:55:56.6787146Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-24T01:55:56.6792413Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68501/merge to s
2020-01-24T01:55:56.6793883Z Task         : Get sources
2020-01-24T01:55:56.6793958Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-24T01:55:56.6793991Z Version      : 1.0.0
2020-01-24T01:55:56.6794026Z Author       : Microsoft
---
2020-01-24T01:55:57.4350990Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-24T01:55:57.4387775Z ##[command]git config gc.auto 0
2020-01-24T01:55:57.4390904Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-24T01:55:57.4393569Z ##[command]git config --get-all http.proxy
2020-01-24T01:55:57.4461234Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68501/merge:refs/remotes/pull/68501/merge
---
2020-01-24T02:01:49.3422355Z 
2020-01-24T02:01:49.3758806Z error: could not compile `rustc`.
2020-01-24T02:01:49.3758988Z 
2020-01-24T02:01:49.3759748Z To learn more, run the command again with --verbose.
2020-01-24T02:01:49.3809747Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-24T02:01:49.3810458Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-01-24T02:01:49.3810639Z Build completed unsuccessfully in 0:03:44
2020-01-24T02:01:49.3858619Z == clock drift check ==
2020-01-24T02:01:49.3868624Z   local time: Fri Jan 24 02:01:49 UTC 2020
2020-01-24T02:01:49.3868624Z   local time: Fri Jan 24 02:01:49 UTC 2020
2020-01-24T02:01:49.4778948Z   network time: Fri, 24 Jan 2020 02:01:49 GMT
2020-01-24T02:01:49.4799351Z == end clock drift check ==
2020-01-24T02:01:49.7778960Z 
2020-01-24T02:01:49.7835632Z ##[error]Bash exited with code '1'.
2020-01-24T02:01:49.7848119Z ##[section]Finishing: Run build
2020-01-24T02:01:49.7861038Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68501/merge to s
2020-01-24T02:01:49.7862664Z Task         : Get sources
2020-01-24T02:01:49.7862726Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-24T02:01:49.7862784Z Version      : 1.0.0
2020-01-24T02:01:49.7862826Z Author       : Microsoft
2020-01-24T02:01:49.7862826Z Author       : Microsoft
2020-01-24T02:01:49.7862871Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-24T02:01:49.7862934Z ==============================================================================
2020-01-24T02:01:50.1009981Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-24T02:01:50.1053595Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68501/merge to s
2020-01-24T02:01:50.1198866Z Cleaning up task key
2020-01-24T02:01:50.1199617Z Start cleaning up orphan processes.
2020-01-24T02:01:50.1422728Z Terminate orphan process: pid (3796) (python)
2020-01-24T02:01:50.1435863Z ##[section]Finishing: Finalize Job
