plain
2020-04-17T18:55:49.3082277Z ========================== Starting Command Output ===========================
2020-04-17T18:55:49.3085533Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a82dc83f-162c-4170-9e92-e9adcfe8d91a.sh
2020-04-17T18:55:49.3085914Z 
2020-04-17T18:55:49.3090012Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-17T18:55:49.3107185Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71256/merge to s
2020-04-17T18:55:49.3109748Z Task         : Get sources
2020-04-17T18:55:49.3109956Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-17T18:55:49.3110142Z Version      : 1.0.0
2020-04-17T18:55:49.3110269Z Author       : Microsoft
---
2020-04-17T18:55:50.3067862Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-17T18:55:50.3072917Z ##[command]git config gc.auto 0
2020-04-17T18:55:50.3076447Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-17T18:55:50.3079696Z ##[command]git config --get-all http.proxy
2020-04-17T18:55:50.3085577Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71256/merge:refs/remotes/pull/71256/merge
---
2020-04-17T18:59:18.4040222Z  ---> 78ad2f4d4aca
2020-04-17T18:59:18.4040439Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-17T18:59:18.4043116Z  ---> Using cache
2020-04-17T18:59:18.4043789Z  ---> 4d2dc61c4d00
2020-04-17T18:59:18.4045288Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-17T18:59:18.4049612Z  ---> 776b6266a8b7
2020-04-17T18:59:18.4082940Z Successfully built 776b6266a8b7
2020-04-17T18:59:18.4118079Z Successfully tagged rust-ci:latest
2020-04-17T18:59:18.4658194Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-17T18:59:18.4658194Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-17T18:59:18.4673266Z Looks like docker image is the same as before, not uploading
2020-04-17T18:59:23.8559338Z [CI_JOB_NAME=mingw-check]
2020-04-17T18:59:23.8779725Z [CI_JOB_NAME=mingw-check]
2020-04-17T18:59:23.8808606Z == clock drift check ==
2020-04-17T18:59:23.8814576Z   local time: Fri Apr 17 18:59:23 UTC 2020
2020-04-17T18:59:24.1706226Z   network time: Fri, 17 Apr 2020 18:59:24 GMT
2020-04-17T18:59:24.1732276Z Starting sccache server...
2020-04-17T18:59:24.2848564Z configure: processing command line
2020-04-17T18:59:24.2848854Z configure: 
2020-04-17T18:59:24.2849819Z configure: rust.parallel-compiler := True
---
2020-04-17T19:01:31.8168575Z expected success, got: exit code: 101
2020-04-17T19:01:31.8168848Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-17T19:01:31.8169145Z Build completed unsuccessfully in 0:02:07
2020-04-17T19:01:31.8277470Z == clock drift check ==
2020-04-17T19:01:31.8289299Z   local time: Fri Apr 17 19:01:31 UTC 2020
2020-04-17T19:01:32.0141731Z   network time: Fri, 17 Apr 2020 19:01:32 GMT
2020-04-17T19:01:33.0222221Z 
2020-04-17T19:01:33.0222221Z 
2020-04-17T19:01:33.0277426Z ##[error]Bash exited with code '1'.
2020-04-17T19:01:33.0291180Z ##[section]Finishing: Run build
2020-04-17T19:01:33.0338660Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71256/merge to s
2020-04-17T19:01:33.0343659Z Task         : Get sources
2020-04-17T19:01:33.0343974Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-17T19:01:33.0344265Z Version      : 1.0.0
2020-04-17T19:01:33.0344488Z Author       : Microsoft
2020-04-17T19:01:33.0344488Z Author       : Microsoft
2020-04-17T19:01:33.0344827Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-17T19:01:33.0345194Z ==============================================================================
2020-04-17T19:01:33.3660413Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-17T19:01:33.3706165Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71256/merge to s
2020-04-17T19:01:33.3793927Z Cleaning up task key
2020-04-17T19:01:33.3795078Z Start cleaning up orphan processes.
2020-04-17T19:01:33.3970887Z Terminate orphan process: pid (3471) (python)
2020-04-17T19:01:33.4148207Z ##[section]Finishing: Finalize Job
