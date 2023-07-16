plain
2020-04-27T04:47:43.5067524Z ========================== Starting Command Output ===========================
2020-04-27T04:47:43.5070491Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/275a8ecf-d203-47cf-b342-450176940255.sh
2020-04-27T04:47:43.5070748Z 
2020-04-27T04:47:43.5074559Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-27T04:47:43.5090788Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71158/merge to s
2020-04-27T04:47:43.5093660Z Task         : Get sources
2020-04-27T04:47:43.5093919Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-27T04:47:43.5094190Z Version      : 1.0.0
2020-04-27T04:47:43.5094364Z Author       : Microsoft
---
2020-04-27T04:47:44.7469424Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-27T04:47:44.7479057Z ##[command]git config gc.auto 0
2020-04-27T04:47:44.7484106Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-27T04:47:44.7488817Z ##[command]git config --get-all http.proxy
2020-04-27T04:47:44.7497576Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71158/merge:refs/remotes/pull/71158/merge
---
2020-04-27T04:51:20.6166573Z  ---> f7353ccad5b1
2020-04-27T04:51:20.6166992Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-27T04:51:20.6168610Z  ---> Using cache
2020-04-27T04:51:20.6169201Z  ---> ed38efbaa060
2020-04-27T04:51:20.6170624Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-27T04:51:20.6173548Z  ---> c5008ef7ae8e
2020-04-27T04:51:20.6215099Z Successfully built c5008ef7ae8e
2020-04-27T04:51:20.6240412Z Successfully tagged rust-ci:latest
2020-04-27T04:51:20.6895883Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-27T04:51:20.6895883Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-27T04:51:20.6911688Z Looks like docker image is the same as before, not uploading
2020-04-27T04:51:29.3688291Z [CI_JOB_NAME=mingw-check]
2020-04-27T04:51:29.3941606Z [CI_JOB_NAME=mingw-check]
2020-04-27T04:51:29.3958378Z == clock drift check ==
2020-04-27T04:51:29.3967075Z   local time: Mon Apr 27 04:51:29 UTC 2020
2020-04-27T04:51:29.6750963Z   network time: Mon, 27 Apr 2020 04:51:29 GMT
2020-04-27T04:51:29.6778952Z Starting sccache server...
2020-04-27T04:51:29.7882540Z configure: processing command line
2020-04-27T04:51:29.7882823Z configure: 
2020-04-27T04:51:29.7884269Z configure: rust.parallel-compiler := True
---
2020-04-27T04:53:38.1099297Z     Checking unicode-width v0.1.6
2020-04-27T04:53:38.1863968Z     Checking getopts v0.2.21
2020-04-27T04:53:40.1784543Z     Checking test v0.0.0 (/checkout/src/libtest)
2020-04-27T04:53:40.8386652Z     Finished release [optimized] target(s) in 25.91s
2020-04-27T04:53:40.8390933Z {"reason":"build-finished","success":true}
2020-04-27T04:53:41.8405142Z     Checking cfg-if v0.1.10
2020-04-27T04:53:41.8407374Z    Compiling libc v0.2.69
2020-04-27T04:53:41.8408219Z    Compiling semver-parser v0.7.0
2020-04-27T04:53:42.3036644Z     Checking lazy_static v1.4.0
---
2020-04-27T04:55:13.8010315Z error: could not compile `fmt_macros`.
2020-04-27T04:55:13.8015483Z 
2020-04-27T04:55:13.8016502Z To learn more, run the command again with --verbose.
2020-04-27T04:55:13.8032628Z warning: build failed, waiting for other jobs to finish...
2020-04-27T04:55:14.3170886Z {"reason":"build-finished","success":false}
2020-04-27T04:55:14.3284961Z error: build failed
2020-04-27T04:55:14.3315876Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-27T04:55:14.3331022Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-27T04:55:14.3331445Z Build completed unsuccessfully in 0:03:44
2020-04-27T04:55:14.3438500Z == clock drift check ==
2020-04-27T04:55:14.3464555Z   local time: Mon Apr 27 04:55:14 UTC 2020
2020-04-27T04:55:14.3464555Z   local time: Mon Apr 27 04:55:14 UTC 2020
2020-04-27T04:55:14.4320995Z   network time: Mon, 27 Apr 2020 04:55:14 GMT
2020-04-27T04:55:15.3121015Z 
2020-04-27T04:55:15.3121015Z 
2020-04-27T04:55:15.3191372Z ##[error]Bash exited with code '1'.
2020-04-27T04:55:15.3203977Z ##[section]Finishing: Run build
2020-04-27T04:55:15.3266680Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71158/merge to s
2020-04-27T04:55:15.3271454Z Task         : Get sources
2020-04-27T04:55:15.3271765Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-27T04:55:15.3272067Z Version      : 1.0.0
2020-04-27T04:55:15.3272274Z Author       : Microsoft
2020-04-27T04:55:15.3272274Z Author       : Microsoft
2020-04-27T04:55:15.3272612Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-27T04:55:15.3272990Z ==============================================================================
2020-04-27T04:55:15.6583458Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-27T04:55:15.6634472Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71158/merge to s
2020-04-27T04:55:15.6722614Z Cleaning up task key
2020-04-27T04:55:15.6723875Z Start cleaning up orphan processes.
2020-04-27T04:55:15.6899491Z Terminate orphan process: pid (3657) (python)
2020-04-27T04:55:15.7101800Z ##[section]Finishing: Finalize Job
