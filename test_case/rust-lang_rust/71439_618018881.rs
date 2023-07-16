plain
2020-04-22T20:18:00.8224243Z ========================== Starting Command Output ===========================
2020-04-22T20:18:00.8227558Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/380eec9d-af1e-4452-8564-082ef38eb63e.sh
2020-04-22T20:18:00.8227842Z 
2020-04-22T20:18:00.8232736Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-22T20:18:00.8253025Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71439/merge to s
2020-04-22T20:18:00.8256319Z Task         : Get sources
2020-04-22T20:18:00.8256607Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-22T20:18:00.8256885Z Version      : 1.0.0
2020-04-22T20:18:00.8257074Z Author       : Microsoft
---
2020-04-22T20:18:01.8140502Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-22T20:18:01.8146749Z ##[command]git config gc.auto 0
2020-04-22T20:18:01.8151087Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-22T20:18:01.8154825Z ##[command]git config --get-all http.proxy
2020-04-22T20:18:01.8161810Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71439/merge:refs/remotes/pull/71439/merge
---
2020-04-22T20:21:11.7055242Z  ---> 78ad2f4d4aca
2020-04-22T20:21:11.7055469Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-22T20:21:11.7057180Z  ---> Using cache
2020-04-22T20:21:11.7057918Z  ---> 4d2dc61c4d00
2020-04-22T20:21:11.7059437Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-22T20:21:11.7062448Z  ---> 776b6266a8b7
2020-04-22T20:21:11.7099181Z Successfully built 776b6266a8b7
2020-04-22T20:21:11.7144533Z Successfully tagged rust-ci:latest
2020-04-22T20:21:11.8312501Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-22T20:21:11.8312501Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-22T20:21:11.8328738Z Looks like docker image is the same as before, not uploading
2020-04-22T20:21:18.5424088Z [CI_JOB_NAME=mingw-check]
2020-04-22T20:21:18.5743969Z [CI_JOB_NAME=mingw-check]
2020-04-22T20:21:18.5776218Z == clock drift check ==
2020-04-22T20:21:18.5791938Z   local time: Wed Apr 22 20:21:18 UTC 2020
2020-04-22T20:21:18.6743225Z   network time: Wed, 22 Apr 2020 20:21:18 GMT
2020-04-22T20:21:18.6774168Z Starting sccache server...
2020-04-22T20:21:18.7943456Z configure: processing command line
2020-04-22T20:21:18.7944972Z configure: 
2020-04-22T20:21:18.7946015Z configure: rust.parallel-compiler := True
---
2020-04-22T20:21:51.6164384Z   Read-only file system (os error 30)
2020-04-22T20:21:51.6193145Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
2020-04-22T20:21:51.6193886Z Build completed unsuccessfully in 0:00:32
2020-04-22T20:21:51.6322242Z == clock drift check ==
2020-04-22T20:21:51.6336602Z   local time: Wed Apr 22 20:21:51 UTC 2020
2020-04-22T20:21:51.9224125Z   network time: Wed, 22 Apr 2020 20:21:51 GMT
2020-04-22T20:21:59.8575769Z 
2020-04-22T20:21:59.8575769Z 
2020-04-22T20:21:59.8651727Z ##[error]Bash exited with code '1'.
2020-04-22T20:21:59.8665931Z ##[section]Finishing: Run build
2020-04-22T20:21:59.8709269Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71439/merge to s
2020-04-22T20:21:59.8714360Z Task         : Get sources
2020-04-22T20:21:59.8714706Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-22T20:21:59.8715008Z Version      : 1.0.0
2020-04-22T20:21:59.8715222Z Author       : Microsoft
2020-04-22T20:21:59.8715222Z Author       : Microsoft
2020-04-22T20:21:59.8715580Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-22T20:21:59.8715959Z ==============================================================================
2020-04-22T20:22:00.2114348Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-22T20:22:00.2161579Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71439/merge to s
2020-04-22T20:22:00.2251880Z Cleaning up task key
2020-04-22T20:22:00.2253305Z Start cleaning up orphan processes.
2020-04-22T20:22:00.2448827Z Terminate orphan process: pid (3833) (python)
2020-04-22T20:22:00.2579159Z ##[section]Finishing: Finalize Job
