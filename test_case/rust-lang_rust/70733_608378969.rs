plain
2020-04-03T11:15:28.0561040Z ========================== Starting Command Output ===========================
2020-04-03T11:15:28.0563526Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/864e5e47-0a09-49fb-b61f-d7acbc79684c.sh
2020-04-03T11:15:28.0563892Z 
2020-04-03T11:15:28.0568964Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-03T11:15:28.0587931Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70733/merge to s
2020-04-03T11:15:28.0591148Z Task         : Get sources
2020-04-03T11:15:28.0591458Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-03T11:15:28.0591758Z Version      : 1.0.0
2020-04-03T11:15:28.0591978Z Author       : Microsoft
---
2020-04-03T11:15:29.0492975Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-03T11:15:29.0499804Z ##[command]git config gc.auto 0
2020-04-03T11:15:29.0504933Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-03T11:15:29.0509731Z ##[command]git config --get-all http.proxy
2020-04-03T11:15:29.0517078Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70733/merge:refs/remotes/pull/70733/merge
---
2020-04-03T11:17:58.0244746Z  ---> 3fc1b512c57b
2020-04-03T11:17:58.0244986Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-03T11:17:58.0245362Z  ---> Using cache
2020-04-03T11:17:58.0245689Z  ---> 5ee4295733f4
2020-04-03T11:17:58.0247037Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-03T11:17:58.0248767Z  ---> 3d07a0fa42fe
2020-04-03T11:17:58.0275677Z Successfully built 3d07a0fa42fe
2020-04-03T11:17:58.0336956Z Successfully tagged rust-ci:latest
2020-04-03T11:17:58.0852020Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-03T11:17:58.0852020Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-03T11:17:58.0867983Z Looks like docker image is the same as before, not uploading
2020-04-03T11:18:04.8231560Z [CI_JOB_NAME=mingw-check]
2020-04-03T11:18:04.8613682Z [CI_JOB_NAME=mingw-check]
2020-04-03T11:18:04.8625386Z == clock drift check ==
2020-04-03T11:18:04.8634715Z   local time: Fri Apr  3 11:18:04 UTC 2020
2020-04-03T11:18:04.9304595Z   network time: Fri, 03 Apr 2020 11:18:04 GMT
2020-04-03T11:18:04.9331593Z Starting sccache server...
2020-04-03T11:18:05.0212032Z configure: processing command line
2020-04-03T11:18:05.0212345Z configure: 
2020-04-03T11:18:05.0213233Z configure: rust.parallel-compiler := True
---
2020-04-03T11:19:52.4888103Z     Checking alloc v0.0.0 (/checkout/src/liballoc)
2020-04-03T11:19:52.7441881Z error[E0424]: expected value, found module `self`
2020-04-03T11:19:52.7442586Z    --> src/liballoc/sync.rs:769:24
2020-04-03T11:19:52.7443113Z     |
2020-04-03T11:19:52.7443886Z 757 | /     pub unsafe fn increment_strong_count(this: &Self) {
2020-04-03T11:19:52.7445102Z 758 | |         // Using a relaxed ordering is alright here, as knowledge of the
2020-04-03T11:19:52.7446155Z 759 | |         // original reference prevents other threads from erroneously deleting
2020-04-03T11:19:52.7447705Z ...   |
2020-04-03T11:19:52.7447705Z ...   |
2020-04-03T11:19:52.7448505Z 769 | |         let old_size = self.inner().strong.fetch_add(1, Relaxed);
2020-04-03T11:19:52.7449649Z     | |                        ^^^^ `self` value is a keyword only available in methods with a `self` parameter
2020-04-03T11:19:52.7451134Z 782 | |         }
2020-04-03T11:19:52.7451876Z 783 | |     }
2020-04-03T11:19:52.7452696Z     | |_____- this function doesn't have a `self` parameter
2020-04-03T11:19:52.7453069Z 
2020-04-03T11:19:52.7453069Z 
2020-04-03T11:19:52.7471326Z error[E0424]: expected value, found module `self`
2020-04-03T11:19:52.7472005Z    --> src/liballoc/sync.rs:791:9
2020-04-03T11:19:52.7472531Z     |
2020-04-03T11:19:52.7473502Z 788 | /     pub unsafe fn decrement_strong_count(this: &Self) {
2020-04-03T11:19:52.7474629Z 789 | |         // Because `fetch_sub` is already atomic, we do not need to synchronize
2020-04-03T11:19:52.7475681Z 790 | |         // with other threads unless we are going to delete the object.
2020-04-03T11:19:52.7476645Z 791 | |         self.inner().strong.fetch_sub(1, Release);
2020-04-03T11:19:52.7477739Z     | |         ^^^^ `self` value is a keyword only available in methods with a `self` parameter
2020-04-03T11:19:52.7479499Z     | |_____- this function doesn't have a `self` parameter
2020-04-03T11:19:52.7479855Z 
2020-04-03T11:19:52.8786523Z error[E0545]: `issue` must be a non-zero numeric string or "none"
2020-04-03T11:19:52.8787275Z    --> src/liballoc/sync.rs:756:46
2020-04-03T11:19:52.8787275Z    --> src/liballoc/sync.rs:756:46
2020-04-03T11:19:52.8787773Z     |
2020-04-03T11:19:52.8788448Z 756 |     #[unstable(feature = "arc_mutate_count", issue = "0")]
2020-04-03T11:19:52.8790134Z     |                                                      |
2020-04-03T11:19:52.8790134Z     |                                                      |
2020-04-03T11:19:52.8791082Z     |                                                      `issue` must not be "0", use "none" instead
2020-04-03T11:19:52.8792013Z error[E0545]: `issue` must be a non-zero numeric string or "none"
2020-04-03T11:19:52.8792680Z    --> src/liballoc/sync.rs:787:46
2020-04-03T11:19:52.8793170Z     |
2020-04-03T11:19:52.8793170Z     |
2020-04-03T11:19:52.8793819Z 787 |     #[unstable(feature = "arc_mutate_count", issue = "0")]
2020-04-03T11:19:52.8795510Z     |                                                      |
2020-04-03T11:19:52.8795510Z     |                                                      |
2020-04-03T11:19:52.8796416Z     |                                                      `issue` must not be "0", use "none" instead
2020-04-03T11:19:53.5983802Z     Checking rustc-demangle v0.1.16
2020-04-03T11:19:53.7338821Z error[E0599]: no method named `increment_strong_count` found for reference `&sync::Arc<T>` in the current scope
2020-04-03T11:19:53.7339679Z     --> src/liballoc/sync.rs:1016:23
2020-04-03T11:19:53.7340213Z      |
---
2020-04-03T11:19:53.8811183Z expected success, got: exit code: 101
2020-04-03T11:19:53.8811608Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-03T11:19:53.8811953Z Build completed unsuccessfully in 0:01:48
2020-04-03T11:19:53.8864681Z == clock drift check ==
2020-04-03T11:19:53.8875465Z   local time: Fri Apr  3 11:19:53 UTC 2020
2020-04-03T11:19:54.0516021Z   network time: Fri, 03 Apr 2020 11:19:54 GMT
2020-04-03T11:19:55.1030745Z 
2020-04-03T11:19:55.1030745Z 
2020-04-03T11:19:55.1100617Z ##[error]Bash exited with code '1'.
2020-04-03T11:19:55.1114291Z ##[section]Finishing: Run build
2020-04-03T11:19:55.1158257Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70733/merge to s
2020-04-03T11:19:55.1163116Z Task         : Get sources
2020-04-03T11:19:55.1163472Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-03T11:19:55.1163798Z Version      : 1.0.0
2020-04-03T11:19:55.1164051Z Author       : Microsoft
2020-04-03T11:19:55.1164051Z Author       : Microsoft
2020-04-03T11:19:55.1164652Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-03T11:19:55.1165077Z ==============================================================================
2020-04-03T11:19:55.4507719Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-03T11:19:55.4551405Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70733/merge to s
2020-04-03T11:19:55.4645150Z Cleaning up task key
2020-04-03T11:19:55.4646682Z Start cleaning up orphan processes.
2020-04-03T11:19:55.4834392Z Terminate orphan process: pid (3552) (python)
2020-04-03T11:19:55.4965122Z ##[section]Finishing: Finalize Job
