plain
2020-04-03T18:30:57.9852724Z ========================== Starting Command Output ===========================
2020-04-03T18:30:57.9858992Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0d7351fd-18f7-4e9a-a05b-17f4ec30b8bb.sh
2020-04-03T18:30:57.9859285Z 
2020-04-03T18:30:57.9863935Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-03T18:30:57.9884744Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70745/merge to s
2020-04-03T18:30:57.9888853Z Task         : Get sources
2020-04-03T18:30:57.9889118Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-03T18:30:57.9889376Z Version      : 1.0.0
2020-04-03T18:30:57.9889574Z Author       : Microsoft
---
2020-04-03T18:30:58.9894152Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-03T18:30:58.9899314Z ##[command]git config gc.auto 0
2020-04-03T18:30:58.9902768Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-03T18:30:58.9906201Z ##[command]git config --get-all http.proxy
2020-04-03T18:30:58.9911761Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70745/merge:refs/remotes/pull/70745/merge
---
2020-04-03T18:33:20.2216138Z  ---> 3fc1b512c57b
2020-04-03T18:33:20.2216429Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-03T18:33:20.2217265Z  ---> Using cache
2020-04-03T18:33:20.2217692Z  ---> 5ee4295733f4
2020-04-03T18:33:20.2221923Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-03T18:33:20.2223536Z  ---> 3d07a0fa42fe
2020-04-03T18:33:20.2273531Z Successfully built 3d07a0fa42fe
2020-04-03T18:33:20.2318347Z Successfully tagged rust-ci:latest
2020-04-03T18:33:20.2628929Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-03T18:33:20.2628929Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-03T18:33:20.2643439Z Looks like docker image is the same as before, not uploading
2020-04-03T18:33:25.9034403Z [CI_JOB_NAME=mingw-check]
2020-04-03T18:33:25.9290263Z [CI_JOB_NAME=mingw-check]
2020-04-03T18:33:25.9322811Z == clock drift check ==
2020-04-03T18:33:25.9336654Z   local time: Fri Apr  3 18:33:25 UTC 2020
2020-04-03T18:33:26.0917461Z   network time: Fri, 03 Apr 2020 18:33:26 GMT
2020-04-03T18:33:26.0944425Z Starting sccache server...
2020-04-03T18:33:26.1871449Z configure: processing command line
2020-04-03T18:33:26.1873797Z configure: 
2020-04-03T18:33:26.1875057Z configure: rust.parallel-compiler := True
---
2020-04-03T18:33:36.7585453Z 
2020-04-03T18:33:36.7618083Z #############################################################             85.1%
2020-04-03T18:33:36.7618573Z ######################################################################## 100.0%
2020-04-03T18:33:37.0723307Z extracting /checkout/obj/build/cache/2020-01-31/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz
2020-04-03T18:33:37.1071312Z error: failed to read `/rust-analyzer/crates/ra_syntax/Cargo.toml`
2020-04-03T18:33:37.1071802Z Caused by:
2020-04-03T18:33:37.1072034Z   No such file or directory (os error 2)
2020-04-03T18:33:37.1076825Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
2020-04-03T18:33:37.1077380Z Build completed unsuccessfully in 0:00:10
2020-04-03T18:33:37.1077380Z Build completed unsuccessfully in 0:00:10
2020-04-03T18:33:37.1119764Z == clock drift check ==
2020-04-03T18:33:37.1133881Z   local time: Fri Apr  3 18:33:37 UTC 2020
2020-04-03T18:33:37.1969621Z   network time: Fri, 03 Apr 2020 18:33:37 GMT
2020-04-03T18:33:44.9241587Z 
2020-04-03T18:33:44.9241587Z 
2020-04-03T18:33:44.9289682Z ##[error]Bash exited with code '1'.
2020-04-03T18:33:44.9309192Z ##[section]Finishing: Run build
2020-04-03T18:33:44.9358614Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70745/merge to s
2020-04-03T18:33:44.9364667Z Task         : Get sources
2020-04-03T18:33:44.9365075Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-03T18:33:44.9365441Z Version      : 1.0.0
2020-04-03T18:33:44.9365723Z Author       : Microsoft
2020-04-03T18:33:44.9365723Z Author       : Microsoft
2020-04-03T18:33:44.9366162Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-03T18:33:44.9366646Z ==============================================================================
2020-04-03T18:33:45.2630286Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-03T18:33:45.2666899Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70745/merge to s
2020-04-03T18:33:45.2771968Z Cleaning up task key
2020-04-03T18:33:45.2773349Z Start cleaning up orphan processes.
2020-04-03T18:33:45.2981105Z Terminate orphan process: pid (3538) (python)
2020-04-03T18:33:45.3133047Z ##[section]Finishing: Finalize Job
