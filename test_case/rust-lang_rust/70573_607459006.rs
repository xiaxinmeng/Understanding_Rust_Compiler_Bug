plain
2020-04-01T19:50:30.0082967Z ========================== Starting Command Output ===========================
2020-04-01T19:50:30.0100714Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d0f3d78a-a7de-49d7-a66c-4b811c55b7fa.sh
2020-04-01T19:50:30.0265424Z 
2020-04-01T19:50:30.0315696Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-01T19:50:30.0330830Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70573/merge to s
2020-04-01T19:50:30.0333483Z Task         : Get sources
2020-04-01T19:50:30.0333699Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-01T19:50:30.0333944Z Version      : 1.0.0
2020-04-01T19:50:30.0334085Z Author       : Microsoft
---
2020-04-01T19:50:30.8324658Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-01T19:50:30.8333908Z ##[command]git config gc.auto 0
2020-04-01T19:50:30.8340342Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-01T19:50:30.8346111Z ##[command]git config --get-all http.proxy
2020-04-01T19:50:30.8355481Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70573/merge:refs/remotes/pull/70573/merge
---
2020-04-01T19:52:32.6566679Z  ---> 3fc1b512c57b
2020-04-01T19:52:32.6567059Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-01T19:52:32.6567364Z  ---> Using cache
2020-04-01T19:52:32.6567625Z  ---> 5ee4295733f4
2020-04-01T19:52:32.6568764Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-01T19:52:32.6570039Z  ---> 3d07a0fa42fe
2020-04-01T19:52:32.6570214Z Successfully built 3d07a0fa42fe
2020-04-01T19:52:32.6570565Z Successfully tagged rust-ci:latest
2020-04-01T19:52:32.6818010Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-01T19:52:32.6818010Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-01T19:52:32.6835534Z Looks like docker image is the same as before, not uploading
2020-04-01T19:52:40.8101703Z [CI_JOB_NAME=mingw-check]
2020-04-01T19:52:40.8364286Z [CI_JOB_NAME=mingw-check]
2020-04-01T19:52:40.8390379Z == clock drift check ==
2020-04-01T19:52:40.8399979Z   local time: Wed Apr  1 19:52:40 UTC 2020
2020-04-01T19:52:40.8780779Z   network time: Wed, 01 Apr 2020 19:52:40 GMT
2020-04-01T19:52:40.8804191Z Starting sccache server...
2020-04-01T19:52:40.9643825Z configure: processing command line
2020-04-01T19:52:40.9644222Z configure: 
2020-04-01T19:52:40.9645170Z configure: rust.parallel-compiler := True
---
2020-04-01T19:54:26.2471237Z 
2020-04-01T19:54:26.2526966Z error[E0282]: type annotations needed
2020-04-01T19:54:26.2546564Z     --> src/liballoc/vec.rs:1310:13
2020-04-01T19:54:26.2548182Z      |
2020-04-01T19:54:26.2550015Z 1310 |             Vec::start_lte_end_drain_assert_failed(start, end);
2020-04-01T19:54:26.2552117Z 
2020-04-01T19:54:26.2567611Z error[E0282]: type annotations needed
2020-04-01T19:54:26.2586891Z     --> src/liballoc/vec.rs:1417:13
2020-04-01T19:54:26.2587679Z      |
2020-04-01T19:54:26.2587679Z      |
2020-04-01T19:54:26.2589147Z 1417 |             Vec::split_off_assert_failed(at, self.len());
2020-04-01T19:54:26.2590504Z 
2020-04-01T19:54:26.3306012Z error: aborting due to 4 previous errors
2020-04-01T19:54:26.3306339Z 
2020-04-01T19:54:26.3306815Z For more information about this error, try `rustc --explain E0282`.
---
2020-04-01T19:54:26.3695875Z expected success, got: exit code: 101
2020-04-01T19:54:26.3699277Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-01T19:54:26.3699581Z Build completed unsuccessfully in 0:01:45
2020-04-01T19:54:26.3749259Z == clock drift check ==
2020-04-01T19:54:26.3761015Z   local time: Wed Apr  1 19:54:26 UTC 2020
2020-04-01T19:54:26.6396190Z   network time: Wed, 01 Apr 2020 19:54:26 GMT
2020-04-01T19:54:27.5478059Z 
2020-04-01T19:54:27.5478059Z 
2020-04-01T19:54:27.5543943Z ##[error]Bash exited with code '1'.
2020-04-01T19:54:27.5554862Z ##[section]Finishing: Run build
2020-04-01T19:54:27.5606772Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70573/merge to s
2020-04-01T19:54:27.5611654Z Task         : Get sources
2020-04-01T19:54:27.5612231Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-01T19:54:27.5612528Z Version      : 1.0.0
2020-04-01T19:54:27.5613122Z Author       : Microsoft
2020-04-01T19:54:27.5613122Z Author       : Microsoft
2020-04-01T19:54:27.5613827Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-01T19:54:27.5614436Z ==============================================================================
2020-04-01T19:54:27.8759107Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-01T19:54:27.8800422Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70573/merge to s
2020-04-01T19:54:27.8876076Z Cleaning up task key
2020-04-01T19:54:27.8877367Z Start cleaning up orphan processes.
2020-04-01T19:54:27.9046197Z Terminate orphan process: pid (5789) (python)
2020-04-01T19:54:27.9175398Z ##[section]Finishing: Finalize Job
