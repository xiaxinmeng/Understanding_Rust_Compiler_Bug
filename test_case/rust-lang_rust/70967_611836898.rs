plain
2020-04-10T01:48:16.1164382Z ========================== Starting Command Output ===========================
2020-04-10T01:48:16.1167629Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/82671211-a012-483e-8c9c-52c5910fff7c.sh
2020-04-10T01:48:16.1168032Z 
2020-04-10T01:48:16.1172577Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-10T01:48:16.1191665Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70967/merge to s
2020-04-10T01:48:16.1194897Z Task         : Get sources
2020-04-10T01:48:16.1195162Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-10T01:48:16.1195631Z Version      : 1.0.0
2020-04-10T01:48:16.1195804Z Author       : Microsoft
---
2020-04-10T01:48:17.1094630Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-10T01:48:17.1099142Z ##[command]git config gc.auto 0
2020-04-10T01:48:17.1102293Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-10T01:48:17.1105121Z ##[command]git config --get-all http.proxy
2020-04-10T01:48:17.1111186Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70967/merge:refs/remotes/pull/70967/merge
---
2020-04-10T01:50:35.1994289Z  ---> 3fc1b512c57b
2020-04-10T01:50:35.1994515Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-10T01:50:35.1994847Z  ---> Using cache
2020-04-10T01:50:35.1995136Z  ---> 5ee4295733f4
2020-04-10T01:50:35.1996354Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-10T01:50:35.1997544Z  ---> 3d07a0fa42fe
2020-04-10T01:50:35.1997744Z Successfully built 3d07a0fa42fe
2020-04-10T01:50:35.2011553Z Successfully tagged rust-ci:latest
2020-04-10T01:50:35.2644748Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-10T01:50:35.2644748Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-10T01:50:35.2662270Z Looks like docker image is the same as before, not uploading
2020-04-10T01:50:43.4626101Z [CI_JOB_NAME=mingw-check]
2020-04-10T01:50:43.4933325Z [CI_JOB_NAME=mingw-check]
2020-04-10T01:50:43.4980500Z == clock drift check ==
2020-04-10T01:50:43.5002903Z   local time: Fri Apr 10 01:50:43 UTC 2020
2020-04-10T01:50:43.6839502Z   network time: Fri, 10 Apr 2020 01:50:43 GMT
2020-04-10T01:50:43.6861896Z Starting sccache server...
2020-04-10T01:50:43.7778979Z configure: processing command line
2020-04-10T01:50:43.7779566Z configure: 
2020-04-10T01:50:43.7780508Z configure: rust.parallel-compiler := True
---
2020-04-10T01:52:54.8512962Z     Checking alloc v0.0.0 (/checkout/src/liballoc)
2020-04-10T01:52:55.1196945Z error[E0412]: cannot find type `MaybeUninit` in this scope
2020-04-10T01:52:55.1197637Z    --> src/liballoc/vec.rs:883:66
2020-04-10T01:52:55.1198060Z     |
2020-04-10T01:52:55.1198711Z 883 |     pub fn get_uninit_unchecked(&mut self, index: usize) -> &mut MaybeUninit<T> {
2020-04-10T01:52:55.1200248Z     |
2020-04-10T01:52:55.1200803Z help: possible candidate is found in another module, you can import it into scope
2020-04-10T01:52:55.1201284Z     |
2020-04-10T01:52:55.1201797Z 62  | use core::mem::MaybeUninit;
2020-04-10T01:52:55.1201797Z 62  | use core::mem::MaybeUninit;
2020-04-10T01:52:55.1202290Z     |
2020-04-10T01:52:55.1206573Z 
2020-04-10T01:52:55.1221490Z error[E0412]: cannot find type `MaybeUninit` in this scope
2020-04-10T01:52:55.1270728Z    --> src/liballoc/vec.rs:887:66
2020-04-10T01:52:55.1310895Z     |
2020-04-10T01:52:55.1322519Z 887 |             unsafe { &mut *(self.as_mut_ptr().add(index) as *mut MaybeUninit<T>) }
2020-04-10T01:52:55.1324879Z     |
2020-04-10T01:52:55.1325770Z help: possible candidate is found in another module, you can import it into scope
2020-04-10T01:52:55.1326482Z     |
2020-04-10T01:52:55.1327354Z 62  | use core::mem::MaybeUninit;
---
2020-04-10T01:52:56.3637366Z expected success, got: exit code: 101
2020-04-10T01:52:56.3653016Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-10T01:52:56.3653391Z Build completed unsuccessfully in 0:02:12
2020-04-10T01:52:56.3705033Z == clock drift check ==
2020-04-10T01:52:56.3723490Z   local time: Fri Apr 10 01:52:56 UTC 2020
2020-04-10T01:52:56.6868353Z   network time: Fri, 10 Apr 2020 01:52:56 GMT
2020-04-10T01:52:57.7321725Z 
2020-04-10T01:52:57.7321725Z 
2020-04-10T01:52:57.7398875Z ##[error]Bash exited with code '1'.
2020-04-10T01:52:57.7413584Z ##[section]Finishing: Run build
2020-04-10T01:52:57.7469796Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70967/merge to s
2020-04-10T01:52:57.7474060Z Task         : Get sources
2020-04-10T01:52:57.7474361Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-10T01:52:57.7474657Z Version      : 1.0.0
2020-04-10T01:52:57.7474851Z Author       : Microsoft
2020-04-10T01:52:57.7474851Z Author       : Microsoft
2020-04-10T01:52:57.7475162Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-10T01:52:57.7475538Z ==============================================================================
2020-04-10T01:52:58.0844948Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-10T01:52:58.0897596Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70967/merge to s
2020-04-10T01:52:58.0984711Z Cleaning up task key
2020-04-10T01:52:58.0986084Z Start cleaning up orphan processes.
2020-04-10T01:52:58.1209469Z Terminate orphan process: pid (3593) (python)
2020-04-10T01:52:58.1340853Z ##[section]Finishing: Finalize Job
