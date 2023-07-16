plain
2020-04-01T19:31:13.9610153Z ========================== Starting Command Output ===========================
2020-04-01T19:31:13.9613507Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/966cf2cc-7524-4150-9c07-ecd98eb4ba14.sh
2020-04-01T19:31:13.9613926Z 
2020-04-01T19:31:13.9618359Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-01T19:31:13.9636294Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70573/merge to s
2020-04-01T19:31:13.9639652Z Task         : Get sources
2020-04-01T19:31:13.9639964Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-01T19:31:13.9640282Z Version      : 1.0.0
2020-04-01T19:31:13.9640488Z Author       : Microsoft
---
2020-04-01T19:31:15.2177567Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-01T19:31:15.2183188Z ##[command]git config gc.auto 0
2020-04-01T19:31:15.2187335Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-01T19:31:15.2190550Z ##[command]git config --get-all http.proxy
2020-04-01T19:31:15.2197047Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70573/merge:refs/remotes/pull/70573/merge
---
2020-04-01T19:33:36.2996502Z  ---> 3fc1b512c57b
2020-04-01T19:33:36.2998296Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-01T19:33:36.3001706Z  ---> Using cache
2020-04-01T19:33:36.3002828Z  ---> 5ee4295733f4
2020-04-01T19:33:36.3005898Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-01T19:33:36.3011862Z  ---> 3d07a0fa42fe
2020-04-01T19:33:36.3046075Z Successfully built 3d07a0fa42fe
2020-04-01T19:33:36.3096076Z Successfully tagged rust-ci:latest
2020-04-01T19:33:36.3354891Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-01T19:33:36.3354891Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-01T19:33:36.3367424Z Looks like docker image is the same as before, not uploading
2020-04-01T19:33:43.0717094Z [CI_JOB_NAME=mingw-check]
2020-04-01T19:33:43.0998324Z [CI_JOB_NAME=mingw-check]
2020-04-01T19:33:43.1027888Z == clock drift check ==
2020-04-01T19:33:43.1035243Z   local time: Wed Apr  1 19:33:43 UTC 2020
2020-04-01T19:33:43.3681318Z   network time: Wed, 01 Apr 2020 19:33:43 GMT
2020-04-01T19:33:43.3707293Z Starting sccache server...
2020-04-01T19:33:43.4545142Z configure: processing command line
2020-04-01T19:33:43.4545355Z configure: 
2020-04-01T19:33:43.4546335Z configure: rust.parallel-compiler := True
---
2020-04-01T19:35:23.1354747Z      |
2020-04-01T19:35:23.1355533Z 1044 |             removal_assert_failed(index, len);
2020-04-01T19:35:23.1356561Z      |             ^^^^^^^^^^^^^^^^^^^^^ not found in this scope
2020-04-01T19:35:23.1357050Z 
2020-04-01T19:35:23.1370328Z error[E0425]: cannot find function `start_lte_end_drain_assert_failed` in this scope
2020-04-01T19:35:23.1371603Z      |
2020-04-01T19:35:23.1371603Z      |
2020-04-01T19:35:23.1372224Z 1310 |             start_lte_end_drain_assert_failed(start, end);
2020-04-01T19:35:23.1373458Z 
2020-04-01T19:35:23.1373458Z 
2020-04-01T19:35:23.1384602Z error[E0425]: cannot find function `end_lte_len_drain_assert_failed` in this scope
2020-04-01T19:35:23.1386220Z      |
2020-04-01T19:35:23.1386220Z      |
2020-04-01T19:35:23.1386836Z 1313 |             end_lte_len_drain_assert_failed(end, len);
2020-04-01T19:35:23.1388054Z 
2020-04-01T19:35:23.1398879Z error[E0425]: cannot find function `split_off_assert_failed` in this scope
2020-04-01T19:35:23.1399498Z     --> src/liballoc/vec.rs:1417:13
2020-04-01T19:35:23.1399934Z      |
2020-04-01T19:35:23.1399934Z      |
2020-04-01T19:35:23.1400460Z 1417 |             split_off_assert_failed(at, self.len());
2020-04-01T19:35:23.1401513Z 
2020-04-01T19:35:23.9744524Z     Checking rustc-demangle v0.1.16
2020-04-01T19:35:24.1351328Z error: aborting due to 5 previous errors
2020-04-01T19:35:24.1351670Z 
---
2020-04-01T19:35:24.2283810Z expected success, got: exit code: 101
2020-04-01T19:35:24.2293449Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-01T19:35:24.2293817Z Build completed unsuccessfully in 0:01:40
2020-04-01T19:35:24.2342724Z == clock drift check ==
2020-04-01T19:35:24.2357330Z   local time: Wed Apr  1 19:35:24 UTC 2020
2020-04-01T19:35:24.3095638Z   network time: Wed, 01 Apr 2020 19:35:24 GMT
2020-04-01T19:35:25.3174933Z 
2020-04-01T19:35:25.3174933Z 
2020-04-01T19:35:25.3241209Z ##[error]Bash exited with code '1'.
2020-04-01T19:35:25.3254463Z ##[section]Finishing: Run build
2020-04-01T19:35:25.3306137Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70573/merge to s
2020-04-01T19:35:25.3311433Z Task         : Get sources
2020-04-01T19:35:25.3311764Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-01T19:35:25.3312086Z Version      : 1.0.0
2020-04-01T19:35:25.3312301Z Author       : Microsoft
2020-04-01T19:35:25.3312301Z Author       : Microsoft
2020-04-01T19:35:25.3312635Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-01T19:35:25.3313042Z ==============================================================================
2020-04-01T19:35:25.6617026Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-01T19:35:25.6659457Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70573/merge to s
2020-04-01T19:35:25.6734970Z Cleaning up task key
2020-04-01T19:35:25.6736190Z Start cleaning up orphan processes.
2020-04-01T19:35:25.7085319Z Terminate orphan process: pid (6066) (python)
2020-04-01T19:35:25.7110489Z ##[section]Finishing: Finalize Job
