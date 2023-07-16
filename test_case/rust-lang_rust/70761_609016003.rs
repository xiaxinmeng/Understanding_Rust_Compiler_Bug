plain
2020-04-04T11:40:44.7295950Z ========================== Starting Command Output ===========================
2020-04-04T11:40:44.7300241Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/53da5855-4fdd-42af-a980-40985059a64a.sh
2020-04-04T11:40:44.7367097Z 
2020-04-04T11:40:44.7372295Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-04T11:40:44.7391785Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70761/merge to s
2020-04-04T11:40:44.7395207Z Task         : Get sources
2020-04-04T11:40:44.7395520Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-04T11:40:44.7395843Z Version      : 1.0.0
2020-04-04T11:40:44.7396050Z Author       : Microsoft
---
2020-04-04T11:40:45.7306696Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-04T11:40:45.7314656Z ##[command]git config gc.auto 0
2020-04-04T11:40:45.7321225Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-04T11:40:45.7327172Z ##[command]git config --get-all http.proxy
2020-04-04T11:40:45.7337206Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70761/merge:refs/remotes/pull/70761/merge
---
2020-04-04T11:42:45.9463925Z  ---> 3fc1b512c57b
2020-04-04T11:42:45.9464162Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-04T11:42:45.9464555Z  ---> Using cache
2020-04-04T11:42:45.9464895Z  ---> 5ee4295733f4
2020-04-04T11:42:45.9466456Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-04T11:42:45.9467812Z  ---> 3d07a0fa42fe
2020-04-04T11:42:45.9468033Z Successfully built 3d07a0fa42fe
2020-04-04T11:42:45.9485184Z Successfully tagged rust-ci:latest
2020-04-04T11:42:45.9754483Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-04T11:42:45.9754483Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-04T11:42:45.9768550Z Looks like docker image is the same as before, not uploading
2020-04-04T11:42:51.3851430Z [CI_JOB_NAME=mingw-check]
2020-04-04T11:42:51.4072408Z [CI_JOB_NAME=mingw-check]
2020-04-04T11:42:51.4089395Z == clock drift check ==
2020-04-04T11:42:51.4103514Z   local time: Sat Apr  4 11:42:51 UTC 2020
2020-04-04T11:42:51.5674208Z   network time: Sat, 04 Apr 2020 11:42:51 GMT
2020-04-04T11:42:51.5700611Z Starting sccache server...
2020-04-04T11:42:51.6431214Z configure: processing command line
2020-04-04T11:42:51.6431435Z configure: 
2020-04-04T11:42:51.6432109Z configure: rust.parallel-compiler := True
---
2020-04-04T11:43:06.7899421Z   Read-only file system (os error 30)
2020-04-04T11:43:06.7918419Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
2020-04-04T11:43:06.7919002Z Build completed unsuccessfully in 0:00:15
2020-04-04T11:43:06.7963011Z == clock drift check ==
2020-04-04T11:43:06.7970726Z   local time: Sat Apr  4 11:43:06 UTC 2020
2020-04-04T11:43:06.8859256Z   network time: Sat, 04 Apr 2020 11:43:06 GMT
2020-04-04T11:43:14.8468520Z 
2020-04-04T11:43:14.8468520Z 
2020-04-04T11:43:14.8527551Z ##[error]Bash exited with code '1'.
2020-04-04T11:43:14.8537272Z ##[section]Finishing: Run build
2020-04-04T11:43:14.8572579Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70761/merge to s
2020-04-04T11:43:14.8577243Z Task         : Get sources
2020-04-04T11:43:14.8577601Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-04T11:43:14.8577912Z Version      : 1.0.0
2020-04-04T11:43:14.8578137Z Author       : Microsoft
2020-04-04T11:43:14.8578137Z Author       : Microsoft
2020-04-04T11:43:14.8578498Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-04T11:43:14.8578898Z ==============================================================================
2020-04-04T11:43:15.0988104Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-04T11:43:15.1023259Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70761/merge to s
2020-04-04T11:43:15.1096820Z Cleaning up task key
2020-04-04T11:43:15.1097887Z Start cleaning up orphan processes.
2020-04-04T11:43:15.1339388Z Terminate orphan process: pid (4143) (python)
2020-04-04T11:43:15.1365504Z ##[section]Finishing: Finalize Job
