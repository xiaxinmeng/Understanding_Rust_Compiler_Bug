plain
2020-01-23T20:42:49.3776988Z ========================== Starting Command Output ===========================
2020-01-23T20:42:49.3778934Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/73103080-a521-4555-b816-d1d6a1c3ae5f.sh
2020-01-23T20:42:49.3778996Z 
2020-01-23T20:42:49.3781775Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-23T20:42:49.3787849Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68493/merge to s
2020-01-23T20:42:49.3789850Z Task         : Get sources
2020-01-23T20:42:49.3789883Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-23T20:42:49.3789916Z Version      : 1.0.0
2020-01-23T20:42:49.3789947Z Author       : Microsoft
---
2020-01-23T20:42:50.4551468Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-23T20:42:50.4562579Z ##[command]git config gc.auto 0
2020-01-23T20:42:50.4564701Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-23T20:42:50.4567216Z ##[command]git config --get-all http.proxy
2020-01-23T20:42:50.4573556Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68493/merge:refs/remotes/pull/68493/merge
---
2020-01-23T21:12:06.2060272Z    Compiling semver v0.9.0
2020-01-23T21:12:08.4568859Z error: failed to run custom build command for `libc v0.2.66`
2020-01-23T21:12:08.4569538Z 
2020-01-23T21:12:08.4570068Z Caused by:
2020-01-23T21:12:08.4570761Z   process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-e240690bd4c21baf/build-script-build` (signal: 11, SIGSEGV: invalid memory reference)
2020-01-23T21:12:11.4467048Z error: build failed
2020-01-23T21:12:11.4492698Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-23T21:12:11.4492887Z expected success, got: exit code: 101
2020-01-23T21:12:11.4507562Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-23T21:12:11.4507562Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-23T21:12:11.4507676Z Build completed unsuccessfully in 0:23:29
2020-01-23T21:12:11.4569964Z == clock drift check ==
2020-01-23T21:12:11.4592650Z   local time: Thu Jan 23 21:12:11 UTC 2020
2020-01-23T21:12:12.0123967Z   network time: Thu, 23 Jan 2020 21:12:12 GMT
2020-01-23T21:12:12.0135482Z == end clock drift check ==
2020-01-23T21:12:12.7750996Z 
2020-01-23T21:12:12.7857581Z ##[error]Bash exited with code '1'.
2020-01-23T21:12:12.7874083Z ##[section]Finishing: Run build
2020-01-23T21:12:12.7893547Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68493/merge to s
2020-01-23T21:12:12.7895376Z Task         : Get sources
2020-01-23T21:12:12.7895426Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-23T21:12:12.7895475Z Version      : 1.0.0
2020-01-23T21:12:12.7895537Z Author       : Microsoft
2020-01-23T21:12:12.7895537Z Author       : Microsoft
2020-01-23T21:12:12.7895585Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-23T21:12:12.7895636Z ==============================================================================
2020-01-23T21:12:13.2544774Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-23T21:12:13.2585430Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68493/merge to s
2020-01-23T21:12:13.2706844Z Cleaning up task key
2020-01-23T21:12:13.2707615Z Start cleaning up orphan processes.
2020-01-23T21:12:13.2816137Z Terminate orphan process: pid (3426) (python)
2020-01-23T21:12:13.3027381Z ##[section]Finishing: Finalize Job
