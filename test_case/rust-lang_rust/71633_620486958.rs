plain
2020-04-28T09:15:33.6216107Z ========================== Starting Command Output ===========================
2020-04-28T09:15:33.6218441Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/da199417-64ac-40e0-8caf-dd725fec7b0b.sh
2020-04-28T09:15:33.6218711Z 
2020-04-28T09:15:33.6222797Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-28T09:15:33.6345825Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71633/merge to s
2020-04-28T09:15:33.6348978Z Task         : Get sources
2020-04-28T09:15:33.6349256Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-28T09:15:33.6349542Z Version      : 1.0.0
2020-04-28T09:15:33.6349731Z Author       : Microsoft
---
2020-04-28T09:15:34.8071334Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-28T09:15:34.8079185Z ##[command]git config gc.auto 0
2020-04-28T09:15:34.8084200Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-28T09:15:34.8088528Z ##[command]git config --get-all http.proxy
2020-04-28T09:15:34.8098540Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71633/merge:refs/remotes/pull/71633/merge
---
2020-04-28T09:19:19.2092735Z  ---> f7353ccad5b1
2020-04-28T09:19:19.2092939Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-28T09:19:19.2096337Z  ---> Using cache
2020-04-28T09:19:19.2114469Z  ---> ed38efbaa060
2020-04-28T09:19:19.2115768Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-28T09:19:19.2120041Z  ---> c5008ef7ae8e
2020-04-28T09:19:19.2148186Z Successfully built c5008ef7ae8e
2020-04-28T09:19:19.2178934Z Successfully tagged rust-ci:latest
2020-04-28T09:19:19.2551810Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-28T09:19:19.2551810Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-28T09:19:19.2566086Z Looks like docker image is the same as before, not uploading
2020-04-28T09:19:20.3022309Z [CI_JOB_NAME=mingw-check]
2020-04-28T09:19:20.3257998Z [CI_JOB_NAME=mingw-check]
2020-04-28T09:19:20.3284016Z == clock drift check ==
2020-04-28T09:19:20.3292894Z   local time: Tue Apr 28 09:19:20 UTC 2020
2020-04-28T09:19:20.6185385Z   network time: Tue, 28 Apr 2020 09:19:20 GMT
2020-04-28T09:19:20.6212781Z Starting sccache server...
2020-04-28T09:19:20.7271665Z configure: processing command line
2020-04-28T09:19:20.7271949Z configure: 
2020-04-28T09:19:20.7272817Z configure: rust.parallel-compiler := True
---
2020-04-28T09:21:20.2681583Z     | ^ unexpected closing delimiter
2020-04-28T09:21:20.2681855Z 
2020-04-28T09:21:20.2760839Z error: aborting due to previous error
2020-04-28T09:21:20.2761171Z 
2020-04-28T09:21:20.2776240Z {"reason":"build-finished","success":false}
2020-04-28T09:21:20.2839087Z 
2020-04-28T09:21:20.2839634Z To learn more, run the command again with --verbose.
2020-04-28T09:21:20.2869360Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-28T09:21:20.2870230Z expected success, got: exit code: 101
2020-04-28T09:21:20.2870230Z expected success, got: exit code: 101
2020-04-28T09:21:20.2886102Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-28T09:21:20.2886709Z Build completed unsuccessfully in 0:01:59
2020-04-28T09:21:20.2997313Z == clock drift check ==
2020-04-28T09:21:20.3014927Z   local time: Tue Apr 28 09:21:20 UTC 2020
2020-04-28T09:21:20.3990677Z   network time: Tue, 28 Apr 2020 09:21:20 GMT
2020-04-28T09:21:21.8470761Z 
2020-04-28T09:21:21.8470761Z 
2020-04-28T09:21:21.8533450Z ##[error]Bash exited with code '1'.
2020-04-28T09:21:21.8547275Z ##[section]Finishing: Run build
2020-04-28T09:21:21.8593858Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71633/merge to s
2020-04-28T09:21:21.8598588Z Task         : Get sources
2020-04-28T09:21:21.8598907Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-28T09:21:21.8599201Z Version      : 1.0.0
2020-04-28T09:21:21.8599430Z Author       : Microsoft
2020-04-28T09:21:21.8599430Z Author       : Microsoft
2020-04-28T09:21:21.8599768Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-28T09:21:21.8600140Z ==============================================================================
2020-04-28T09:21:22.2131252Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-28T09:21:22.2137118Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71633/merge to s
2020-04-28T09:21:22.2223588Z Cleaning up task key
2020-04-28T09:21:22.2224832Z Start cleaning up orphan processes.
2020-04-28T09:21:22.2405661Z Terminate orphan process: pid (3702) (python)
2020-04-28T09:21:22.2701130Z ##[section]Finishing: Finalize Job
