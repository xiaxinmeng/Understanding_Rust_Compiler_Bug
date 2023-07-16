plain
2020-04-10T18:43:26.2414609Z ========================== Starting Command Output ===========================
2020-04-10T18:43:26.2418454Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/278bf842-fe9c-4af1-b1e7-e313ad1d7513.sh
2020-04-10T18:43:26.2419057Z 
2020-04-10T18:43:26.2424994Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-10T18:43:26.2447346Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70999/merge to s
2020-04-10T18:43:26.2450589Z Task         : Get sources
2020-04-10T18:43:26.2450897Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-10T18:43:26.2451150Z Version      : 1.0.0
2020-04-10T18:43:26.2451319Z Author       : Microsoft
---
2020-04-10T18:43:27.9167337Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-10T18:43:27.9683082Z ##[command]git config gc.auto 0
2020-04-10T18:43:27.9722065Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-10T18:43:27.9730037Z ##[command]git config --get-all http.proxy
2020-04-10T18:43:27.9743511Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70999/merge:refs/remotes/pull/70999/merge
---
2020-04-10T18:46:53.7434523Z  ---> 3fc1b512c57b
2020-04-10T18:46:53.7434876Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-10T18:46:53.7435351Z  ---> Using cache
2020-04-10T18:46:53.7435832Z  ---> 5ee4295733f4
2020-04-10T18:46:53.7437280Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-10T18:46:53.7438936Z  ---> 3d07a0fa42fe
2020-04-10T18:46:53.7468862Z Successfully built 3d07a0fa42fe
2020-04-10T18:46:53.7492884Z Successfully tagged rust-ci:latest
2020-04-10T18:46:53.7765443Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-10T18:46:53.7765443Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-10T18:46:53.7783150Z Looks like docker image is the same as before, not uploading
2020-04-10T18:47:02.9924521Z [CI_JOB_NAME=mingw-check]
2020-04-10T18:47:02.9925367Z [CI_JOB_NAME=mingw-check]
2020-04-10T18:47:02.9925623Z == clock drift check ==
2020-04-10T18:47:02.9925922Z   local time: Fri Apr 10 18:47:02 UTC 2020
2020-04-10T18:47:02.9926229Z   network time: Fri, 10 Apr 2020 18:47:02 GMT
2020-04-10T18:47:02.9926719Z Starting sccache server...
2020-04-10T18:47:02.9926940Z configure: processing command line
2020-04-10T18:47:02.9927524Z configure: 
2020-04-10T18:47:02.9928000Z configure: rust.parallel-compiler := True
---
2020-04-10T18:47:31.8193626Z   Read-only file system (os error 30)
2020-04-10T18:47:31.8219384Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
2020-04-10T18:47:31.8219890Z Build completed unsuccessfully in 0:00:29
2020-04-10T18:47:31.8265122Z == clock drift check ==
2020-04-10T18:47:31.8286236Z   local time: Fri Apr 10 18:47:31 UTC 2020
2020-04-10T18:47:31.9255087Z   network time: Fri, 10 Apr 2020 18:47:31 GMT
2020-04-10T18:47:39.8740117Z 
2020-04-10T18:47:39.8740117Z 
2020-04-10T18:47:39.8822479Z ##[error]Bash exited with code '1'.
2020-04-10T18:47:39.8838090Z ##[section]Finishing: Run build
2020-04-10T18:47:39.8887537Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70999/merge to s
2020-04-10T18:47:39.8893151Z Task         : Get sources
2020-04-10T18:47:39.8893513Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-10T18:47:39.8893854Z Version      : 1.0.0
2020-04-10T18:47:39.8894102Z Author       : Microsoft
2020-04-10T18:47:39.8894102Z Author       : Microsoft
2020-04-10T18:47:39.8894470Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-10T18:47:39.8894899Z ==============================================================================
2020-04-10T18:47:40.2533627Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-10T18:47:40.2583648Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70999/merge to s
2020-04-10T18:47:40.2674415Z Cleaning up task key
2020-04-10T18:47:40.2675655Z Start cleaning up orphan processes.
2020-04-10T18:47:40.2870988Z Terminate orphan process: pid (4064) (python)
2020-04-10T18:47:40.3018993Z ##[section]Finishing: Finalize Job
