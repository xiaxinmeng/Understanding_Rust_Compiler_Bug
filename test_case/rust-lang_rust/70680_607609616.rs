plain
2020-04-02T04:05:06.9419656Z ========================== Starting Command Output ===========================
2020-04-02T04:05:06.9422184Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ff10246a-1943-4fb5-9373-ca84465489fd.sh
2020-04-02T04:05:06.9422396Z 
2020-04-02T04:05:06.9425786Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-02T04:05:06.9443037Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70680/merge to s
2020-04-02T04:05:06.9445994Z Task         : Get sources
2020-04-02T04:05:06.9446237Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-02T04:05:06.9446470Z Version      : 1.0.0
2020-04-02T04:05:06.9446626Z Author       : Microsoft
---
2020-04-02T04:05:08.2085037Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-02T04:05:08.2091107Z ##[command]git config gc.auto 0
2020-04-02T04:05:08.2096641Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-02T04:05:08.2102865Z ##[command]git config --get-all http.proxy
2020-04-02T04:05:08.2113562Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70680/merge:refs/remotes/pull/70680/merge
---
2020-04-02T04:07:22.9331798Z  ---> 3fc1b512c57b
2020-04-02T04:07:22.9332054Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-02T04:07:22.9332484Z  ---> Using cache
2020-04-02T04:07:22.9332851Z  ---> 5ee4295733f4
2020-04-02T04:07:22.9334290Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-02T04:07:22.9335771Z  ---> 3d07a0fa42fe
2020-04-02T04:07:22.9362724Z Successfully built 3d07a0fa42fe
2020-04-02T04:07:22.9405398Z Successfully tagged rust-ci:latest
2020-04-02T04:07:23.1072535Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-02T04:07:23.1072535Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-02T04:07:23.1086973Z Looks like docker image is the same as before, not uploading
2020-04-02T04:07:29.8840106Z [CI_JOB_NAME=mingw-check]
2020-04-02T04:07:29.9116329Z [CI_JOB_NAME=mingw-check]
2020-04-02T04:07:29.9151407Z == clock drift check ==
2020-04-02T04:07:29.9161110Z   local time: Thu Apr  2 04:07:29 UTC 2020
2020-04-02T04:07:30.0104135Z   network time: Thu, 02 Apr 2020 04:07:30 GMT
2020-04-02T04:07:30.0149872Z Starting sccache server...
2020-04-02T04:07:30.1082344Z configure: processing command line
2020-04-02T04:07:30.1082624Z configure: 
2020-04-02T04:07:30.1083585Z configure: rust.parallel-compiler := True
---
2020-04-02T04:07:43.3289939Z   No such file or directory (os error 2)
2020-04-02T04:07:43.3291498Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
2020-04-02T04:07:43.3292306Z Build completed unsuccessfully in 0:00:13
2020-04-02T04:07:43.3339152Z == clock drift check ==
2020-04-02T04:07:43.3347607Z   local time: Thu Apr  2 04:07:43 UTC 2020
2020-04-02T04:07:43.4939337Z   network time: Thu, 02 Apr 2020 04:07:43 GMT
2020-04-02T04:07:51.2343189Z 
2020-04-02T04:07:51.2343189Z 
2020-04-02T04:07:51.2449133Z ##[error]Bash exited with code '1'.
2020-04-02T04:07:51.2463248Z ##[section]Finishing: Run build
2020-04-02T04:07:51.2508216Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70680/merge to s
2020-04-02T04:07:51.2513539Z Task         : Get sources
2020-04-02T04:07:51.2513924Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-02T04:07:51.2514255Z Version      : 1.0.0
2020-04-02T04:07:51.2514504Z Author       : Microsoft
2020-04-02T04:07:51.2514504Z Author       : Microsoft
2020-04-02T04:07:51.2514866Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-02T04:07:51.2515289Z ==============================================================================
2020-04-02T04:07:51.6032862Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-02T04:07:51.6083147Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70680/merge to s
2020-04-02T04:07:51.6177833Z Cleaning up task key
2020-04-02T04:07:51.6180811Z Start cleaning up orphan processes.
2020-04-02T04:07:51.6388174Z Terminate orphan process: pid (3442) (python)
2020-04-02T04:07:51.6603007Z ##[section]Finishing: Finalize Job
