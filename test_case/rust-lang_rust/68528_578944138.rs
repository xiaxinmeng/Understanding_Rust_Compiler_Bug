plain
2020-01-27T20:21:04.5368479Z ========================== Starting Command Output ===========================
2020-01-27T20:21:04.5369754Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/aca04d9d-537c-4b24-8796-7fbff2826484.sh
2020-01-27T20:21:04.5369783Z 
2020-01-27T20:21:04.5372250Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-27T20:21:04.5377582Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68528/merge to s
2020-01-27T20:21:04.5379064Z Task         : Get sources
2020-01-27T20:21:04.5379098Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-27T20:21:04.5379130Z Version      : 1.0.0
2020-01-27T20:21:04.5379164Z Author       : Microsoft
---
2020-01-27T20:21:05.2860022Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-27T20:21:05.2971919Z ##[command]git config gc.auto 0
2020-01-27T20:21:05.3032799Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-27T20:21:05.3305672Z ##[command]git config --get-all http.proxy
2020-01-27T20:21:05.3311780Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68528/merge:refs/remotes/pull/68528/merge
---
2020-01-27T20:44:30.2345721Z    Compiling semver v0.9.0
2020-01-27T20:44:31.5412937Z error: failed to run custom build command for `libc v0.2.66`
2020-01-27T20:44:31.5413138Z 
2020-01-27T20:44:31.5413195Z Caused by:
2020-01-27T20:44:31.5413613Z   process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-5a80087bc3043aaf/build-script-build` (signal: 11, SIGSEGV: invalid memory reference)
2020-01-27T20:44:31.6810389Z error: build failed
2020-01-27T20:44:31.6829868Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-27T20:44:31.6830056Z expected success, got: exit code: 101
2020-01-27T20:44:31.6841193Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-27T20:44:31.6841193Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-27T20:44:31.6841695Z Build completed unsuccessfully in 0:18:24
2020-01-27T20:44:31.6898291Z == clock drift check ==
2020-01-27T20:44:31.6917474Z   local time: Mon Jan 27 20:44:31 UTC 2020
2020-01-27T20:44:31.9886219Z   network time: Mon, 27 Jan 2020 20:44:31 GMT
2020-01-27T20:44:31.9895251Z == end clock drift check ==
2020-01-27T20:44:32.6671924Z 
2020-01-27T20:44:32.6745498Z ##[error]Bash exited with code '1'.
2020-01-27T20:44:32.6754707Z ##[section]Finishing: Run build
2020-01-27T20:44:32.6775179Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68528/merge to s
2020-01-27T20:44:32.6777211Z Task         : Get sources
2020-01-27T20:44:32.6777260Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-27T20:44:32.6777306Z Version      : 1.0.0
2020-01-27T20:44:32.6777364Z Author       : Microsoft
2020-01-27T20:44:32.6777364Z Author       : Microsoft
2020-01-27T20:44:32.6777410Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-27T20:44:32.6777460Z ==============================================================================
2020-01-27T20:44:33.0708132Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-27T20:44:33.0744354Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68528/merge to s
2020-01-27T20:44:33.0844418Z Cleaning up task key
2020-01-27T20:44:33.0844998Z Start cleaning up orphan processes.
2020-01-27T20:44:33.0952041Z Terminate orphan process: pid (3573) (python)
2020-01-27T20:44:33.1122019Z ##[section]Finishing: Finalize Job
