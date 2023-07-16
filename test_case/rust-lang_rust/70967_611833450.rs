plain
2020-04-10T01:35:18.5218185Z ========================== Starting Command Output ===========================
2020-04-10T01:35:18.5220468Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2006f630-1277-46a7-969e-9152e9c7b34b.sh
2020-04-10T01:35:18.5220725Z 
2020-04-10T01:35:18.5224624Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-10T01:35:18.5242668Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70967/merge to s
2020-04-10T01:35:18.5245782Z Task         : Get sources
2020-04-10T01:35:18.5246065Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-10T01:35:18.5246342Z Version      : 1.0.0
2020-04-10T01:35:18.5246541Z Author       : Microsoft
---
2020-04-10T01:35:19.5357451Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-10T01:35:19.5363473Z ##[command]git config gc.auto 0
2020-04-10T01:35:19.5367829Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-10T01:35:19.5372057Z ##[command]git config --get-all http.proxy
2020-04-10T01:35:19.5378436Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70967/merge:refs/remotes/pull/70967/merge
---
2020-04-10T01:37:32.0534053Z  ---> 3fc1b512c57b
2020-04-10T01:37:32.0534259Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-10T01:37:32.0536166Z  ---> Using cache
2020-04-10T01:37:32.0536459Z  ---> 5ee4295733f4
2020-04-10T01:37:32.0537591Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-10T01:37:32.0538712Z  ---> 3d07a0fa42fe
2020-04-10T01:37:32.0589517Z Successfully built 3d07a0fa42fe
2020-04-10T01:37:32.0620778Z Successfully tagged rust-ci:latest
2020-04-10T01:37:32.0877372Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-10T01:37:32.0877372Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-10T01:37:32.0885831Z Looks like docker image is the same as before, not uploading
2020-04-10T01:37:40.2418719Z [CI_JOB_NAME=mingw-check]
2020-04-10T01:37:40.2568769Z [CI_JOB_NAME=mingw-check]
2020-04-10T01:37:40.2597945Z == clock drift check ==
2020-04-10T01:37:40.2613421Z   local time: Fri Apr 10 01:37:40 UTC 2020
2020-04-10T01:37:40.4446065Z   network time: Fri, 10 Apr 2020 01:37:40 GMT
2020-04-10T01:37:40.4479632Z Starting sccache server...
2020-04-10T01:37:40.5254228Z configure: processing command line
2020-04-10T01:37:40.5254535Z configure: 
2020-04-10T01:37:40.5255514Z configure: rust.parallel-compiler := True
---
2020-04-10T01:39:20.5027218Z 
2020-04-10T01:39:20.5027666Z error: mismatched closing delimiter: `)`
2020-04-10T01:39:20.5028247Z    --> src/liballoc/vec.rs:882:69
2020-04-10T01:39:20.5028692Z     |
2020-04-10T01:39:20.5029324Z 882 |     #[unstable(featute = "vec_get_uninit_unchecked"), issue = "none")]
2020-04-10T01:39:20.5030914Z 
2020-04-10T01:39:20.5031461Z error: mismatched closing delimiter: `]`
2020-04-10T01:39:20.5032053Z    --> src/liballoc/vec.rs:882:70
2020-04-10T01:39:20.5032503Z     |
2020-04-10T01:39:20.5032503Z     |
2020-04-10T01:39:20.5033005Z 309 | impl<T> Vec<T> {
2020-04-10T01:39:20.5033714Z     |                - unclosed delimiter
2020-04-10T01:39:20.5034220Z ...
2020-04-10T01:39:20.5034846Z 882 |     #[unstable(featute = "vec_get_uninit_unchecked"), issue = "none")]
2020-04-10T01:39:20.5036175Z 
2020-04-10T01:39:20.5075388Z error: aborting due to 3 previous errors
2020-04-10T01:39:20.5075646Z 
2020-04-10T01:39:20.5115107Z error: could not compile `alloc`.
---
2020-04-10T01:39:21.4281225Z expected success, got: exit code: 101
2020-04-10T01:39:21.4300035Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-10T01:39:21.4300403Z Build completed unsuccessfully in 0:01:40
2020-04-10T01:39:21.4352229Z == clock drift check ==
2020-04-10T01:39:21.4370020Z   local time: Fri Apr 10 01:39:21 UTC 2020
2020-04-10T01:39:21.6150466Z   network time: Fri, 10 Apr 2020 01:39:21 GMT
2020-04-10T01:39:22.6000571Z 
2020-04-10T01:39:22.6000571Z 
2020-04-10T01:39:22.6066084Z ##[error]Bash exited with code '1'.
2020-04-10T01:39:22.6077785Z ##[section]Finishing: Run build
2020-04-10T01:39:22.6126165Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70967/merge to s
2020-04-10T01:39:22.6130926Z Task         : Get sources
2020-04-10T01:39:22.6131251Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-10T01:39:22.6131567Z Version      : 1.0.0
2020-04-10T01:39:22.6131777Z Author       : Microsoft
2020-04-10T01:39:22.6131777Z Author       : Microsoft
2020-04-10T01:39:22.6132107Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-10T01:39:22.6132504Z ==============================================================================
2020-04-10T01:39:22.9222203Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-10T01:39:22.9265613Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70967/merge to s
2020-04-10T01:39:22.9350285Z Cleaning up task key
2020-04-10T01:39:22.9351598Z Start cleaning up orphan processes.
2020-04-10T01:39:22.9522962Z Terminate orphan process: pid (4080) (python)
2020-04-10T01:39:22.9648117Z ##[section]Finishing: Finalize Job
