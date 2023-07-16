plain
2020-04-16T13:57:20.8893715Z ========================== Starting Command Output ===========================
2020-04-16T13:57:20.8896610Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1200255b-5a41-485a-8904-fcb320c8cf1c.sh
2020-04-16T13:57:20.8896854Z 
2020-04-16T13:57:20.8900483Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-16T13:57:20.8918980Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71199/merge to s
2020-04-16T13:57:20.8921998Z Task         : Get sources
2020-04-16T13:57:20.8922293Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-16T13:57:20.8922562Z Version      : 1.0.0
2020-04-16T13:57:20.8922748Z Author       : Microsoft
---
2020-04-16T13:57:21.8965960Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-16T13:57:21.8970852Z ##[command]git config gc.auto 0
2020-04-16T13:57:21.8974080Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-16T13:57:21.8977032Z ##[command]git config --get-all http.proxy
2020-04-16T13:57:21.8982743Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71199/merge:refs/remotes/pull/71199/merge
---
2020-04-16T14:01:01.5767074Z  ---> 78ad2f4d4aca
2020-04-16T14:01:01.5767360Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-16T14:01:01.5770607Z  ---> Using cache
2020-04-16T14:01:01.5771771Z  ---> 4d2dc61c4d00
2020-04-16T14:01:01.5775479Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-16T14:01:01.5777271Z  ---> 776b6266a8b7
2020-04-16T14:01:01.5858992Z Successfully built 776b6266a8b7
2020-04-16T14:01:01.5905522Z Successfully tagged rust-ci:latest
2020-04-16T14:01:01.6203241Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-16T14:01:01.6203241Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-16T14:01:01.6219969Z Looks like docker image is the same as before, not uploading
2020-04-16T14:01:09.6754127Z [CI_JOB_NAME=mingw-check]
2020-04-16T14:01:09.6987198Z [CI_JOB_NAME=mingw-check]
2020-04-16T14:01:09.7018670Z == clock drift check ==
2020-04-16T14:01:09.7028948Z   local time: Thu Apr 16 14:01:09 UTC 2020
2020-04-16T14:01:09.8825355Z   network time: Thu, 16 Apr 2020 14:01:09 GMT
2020-04-16T14:01:09.8852043Z Starting sccache server...
2020-04-16T14:01:09.9969010Z configure: processing command line
2020-04-16T14:01:09.9969360Z configure: 
2020-04-16T14:01:09.9970382Z configure: rust.parallel-compiler := True
---
2020-04-16T14:04:44.4151290Z 
2020-04-16T14:04:44.4151704Z To learn more, run the command again with --verbose.
2020-04-16T14:04:44.4167828Z warning: build failed, waiting for other jobs to finish...
2020-04-16T14:04:44.7568475Z error: build failed
2020-04-16T14:04:44.7593430Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-16T14:04:44.7600475Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-16T14:04:44.7600802Z Build completed unsuccessfully in 0:03:34
2020-04-16T14:04:44.7704273Z == clock drift check ==
2020-04-16T14:04:44.7704273Z == clock drift check ==
2020-04-16T14:04:44.7725993Z   local time: Thu Apr 16 14:04:44 UTC 2020
2020-04-16T14:04:44.8672792Z   network time: Thu, 16 Apr 2020 14:04:44 GMT
2020-04-16T14:04:45.6224178Z 
2020-04-16T14:04:45.6224178Z 
2020-04-16T14:04:45.6295386Z ##[error]Bash exited with code '1'.
2020-04-16T14:04:45.6308780Z ##[section]Finishing: Run build
2020-04-16T14:04:45.6350260Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71199/merge to s
2020-04-16T14:04:45.6354923Z Task         : Get sources
2020-04-16T14:04:45.6355209Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-16T14:04:45.6355469Z Version      : 1.0.0
2020-04-16T14:04:45.6355673Z Author       : Microsoft
2020-04-16T14:04:45.6355673Z Author       : Microsoft
2020-04-16T14:04:45.6355967Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-16T14:04:45.6356298Z ==============================================================================
2020-04-16T14:04:45.9555379Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-16T14:04:45.9601765Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71199/merge to s
2020-04-16T14:04:45.9689365Z Cleaning up task key
2020-04-16T14:04:45.9691535Z Start cleaning up orphan processes.
2020-04-16T14:04:45.9868608Z Terminate orphan process: pid (3584) (python)
2020-04-16T14:04:46.0019359Z ##[section]Finishing: Finalize Job
