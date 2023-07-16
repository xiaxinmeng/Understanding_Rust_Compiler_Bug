plain
2020-01-24T23:31:31.5806338Z ========================== Starting Command Output ===========================
2020-01-24T23:31:31.5808094Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4c852021-1676-44c1-a5a8-12242c4ae312.sh
2020-01-24T23:31:31.5808267Z 
2020-01-24T23:31:31.5810645Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-24T23:31:31.5815154Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68524/merge to s
2020-01-24T23:31:31.5816383Z Task         : Get sources
2020-01-24T23:31:31.5816409Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-24T23:31:31.5816433Z Version      : 1.0.0
2020-01-24T23:31:31.5816458Z Author       : Microsoft
---
2020-01-24T23:31:32.3727156Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-24T23:31:32.3737131Z ##[command]git config gc.auto 0
2020-01-24T23:31:32.3739093Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-24T23:31:32.3740651Z ##[command]git config --get-all http.proxy
2020-01-24T23:31:32.3745878Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68524/merge:refs/remotes/pull/68524/merge
---
2020-01-24T23:36:10.4032310Z 
2020-01-24T23:36:10.4057756Z error: could not compile `rustc_data_structures`.
2020-01-24T23:36:10.4069232Z warning: build failed, waiting for other jobs to finish...
2020-01-24T23:36:16.3190379Z error: build failed
2020-01-24T23:36:16.3209526Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-24T23:36:16.3220912Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-01-24T23:36:16.3220984Z Build completed unsuccessfully in 0:02:28
2020-01-24T23:36:16.3263549Z == clock drift check ==
2020-01-24T23:36:16.3274475Z   local time: Fri Jan 24 23:36:16 UTC 2020
2020-01-24T23:36:16.3274475Z   local time: Fri Jan 24 23:36:16 UTC 2020
2020-01-24T23:36:16.4823180Z   network time: Fri, 24 Jan 2020 23:36:16 GMT
2020-01-24T23:36:16.4827750Z == end clock drift check ==
2020-01-24T23:36:17.1481592Z 
2020-01-24T23:36:17.1552850Z ##[error]Bash exited with code '1'.
2020-01-24T23:36:17.1561341Z ##[section]Finishing: Run build
2020-01-24T23:36:17.1571776Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68524/merge to s
2020-01-24T23:36:17.1573170Z Task         : Get sources
2020-01-24T23:36:17.1573206Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-24T23:36:17.1573254Z Version      : 1.0.0
2020-01-24T23:36:17.1573298Z Author       : Microsoft
2020-01-24T23:36:17.1573298Z Author       : Microsoft
2020-01-24T23:36:17.1573333Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-24T23:36:17.1573369Z ==============================================================================
2020-01-24T23:36:17.4443264Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-24T23:36:17.4488446Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68524/merge to s
2020-01-24T23:36:17.4600599Z Cleaning up task key
2020-01-24T23:36:17.4601424Z Start cleaning up orphan processes.
2020-01-24T23:36:17.4679205Z Terminate orphan process: pid (3417) (python)
2020-01-24T23:36:17.4829341Z ##[section]Finishing: Finalize Job
