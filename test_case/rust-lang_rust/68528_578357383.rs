plain
2020-01-25T00:29:26.5085565Z ========================== Starting Command Output ===========================
2020-01-25T00:29:26.5090369Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4cf1a6f3-5c62-48f1-912d-d20290d394d7.sh
2020-01-25T00:29:26.5090656Z 
2020-01-25T00:29:26.5093479Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-25T00:29:26.5098584Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68528/merge to s
2020-01-25T00:29:26.5100065Z Task         : Get sources
2020-01-25T00:29:26.5100093Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-25T00:29:26.5100118Z Version      : 1.0.0
2020-01-25T00:29:26.5100143Z Author       : Microsoft
---
2020-01-25T00:29:27.5029435Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-25T00:29:27.5040134Z ##[command]git config gc.auto 0
2020-01-25T00:29:27.5043486Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-25T00:29:27.5046159Z ##[command]git config --get-all http.proxy
2020-01-25T00:29:27.5052754Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68528/merge:refs/remotes/pull/68528/merge
---
2020-01-25T00:52:49.1214028Z    Compiling rustc_index v0.0.0 (/checkout/src/librustc_index)
2020-01-25T00:52:51.2179474Z error: failed to run custom build command for `backtrace-sys v0.1.32`
2020-01-25T00:52:51.2179565Z 
2020-01-25T00:52:51.2179607Z Caused by:
2020-01-25T00:52:51.2180394Z   process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/backtrace-sys-227a27b0f872ec81/build-script-build` (signal: 11, SIGSEGV: invalid memory reference)
2020-01-25T00:52:51.2180827Z --- stdout
2020-01-25T00:52:51.2181082Z cargo:rustc-cfg=rbt
2020-01-25T00:52:51.2187011Z TARGET = Some("x86_64-unknown-linux-gnu")
2020-01-25T00:52:51.2187085Z OPT_LEVEL = Some("2")
2020-01-25T00:52:51.2187294Z HOST = Some("x86_64-unknown-linux-gnu")
2020-01-25T00:52:51.2187512Z CC_x86_64-unknown-linux-gnu = Some("sccache cc")
2020-01-25T00:52:51.2187751Z warning: build failed, waiting for other jobs to finish...
2020-01-25T00:52:51.7306709Z error: build failed
2020-01-25T00:52:51.7324740Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-25T00:52:51.7325391Z expected success, got: exit code: 101
2020-01-25T00:52:51.7325391Z expected success, got: exit code: 101
2020-01-25T00:52:51.7336184Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-25T00:52:51.7337030Z Build completed unsuccessfully in 0:18:22
2020-01-25T00:52:51.7381164Z == clock drift check ==
2020-01-25T00:52:51.7395424Z   local time: Sat Jan 25 00:52:51 UTC 2020
2020-01-25T00:52:52.0071805Z   network time: Sat, 25 Jan 2020 00:52:52 GMT
2020-01-25T00:52:52.0080702Z == end clock drift check ==
2020-01-25T00:52:52.7690170Z 
2020-01-25T00:52:52.7771012Z ##[error]Bash exited with code '1'.
2020-01-25T00:52:52.7789148Z ##[section]Finishing: Run build
2020-01-25T00:52:52.7800641Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68528/merge to s
2020-01-25T00:52:52.7802137Z Task         : Get sources
2020-01-25T00:52:52.7802172Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-25T00:52:52.7802207Z Version      : 1.0.0
2020-01-25T00:52:52.7802254Z Author       : Microsoft
2020-01-25T00:52:52.7802254Z Author       : Microsoft
2020-01-25T00:52:52.7802288Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-25T00:52:52.7802325Z ==============================================================================
2020-01-25T00:52:53.1143397Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-25T00:52:53.1182947Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68528/merge to s
2020-01-25T00:52:53.1268109Z Cleaning up task key
2020-01-25T00:52:53.1268730Z Start cleaning up orphan processes.
2020-01-25T00:52:53.1353669Z Terminate orphan process: pid (5563) (python)
2020-01-25T00:52:53.1604548Z ##[section]Finishing: Finalize Job
