plain
2020-04-03T22:05:55.7408794Z ========================== Starting Command Output ===========================
2020-04-03T22:05:55.7411200Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2e396c19-8908-4474-be19-9a626761bb5c.sh
2020-04-03T22:05:55.7411480Z 
2020-04-03T22:05:55.7415203Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-03T22:05:55.7433368Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70752/merge to s
2020-04-03T22:05:55.7436492Z Task         : Get sources
2020-04-03T22:05:55.7436779Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-03T22:05:55.7437058Z Version      : 1.0.0
2020-04-03T22:05:55.7437264Z Author       : Microsoft
---
2020-04-03T22:05:57.0448280Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-03T22:05:57.0455238Z ##[command]git config gc.auto 0
2020-04-03T22:05:57.0460552Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-03T22:05:57.0466259Z ##[command]git config --get-all http.proxy
2020-04-03T22:05:57.0475675Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70752/merge:refs/remotes/pull/70752/merge
---
2020-04-03T22:08:30.9962678Z  ---> 3fc1b512c57b
2020-04-03T22:08:30.9966434Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-03T22:08:30.9973156Z  ---> Using cache
2020-04-03T22:08:30.9973486Z  ---> 5ee4295733f4
2020-04-03T22:08:30.9974715Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-03T22:08:30.9981585Z  ---> 3d07a0fa42fe
2020-04-03T22:08:31.0014006Z Successfully built 3d07a0fa42fe
2020-04-03T22:08:31.0111611Z Successfully tagged rust-ci:latest
2020-04-03T22:08:31.0384809Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-03T22:08:31.0384809Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-03T22:08:31.0400323Z Looks like docker image is the same as before, not uploading
2020-04-03T22:08:38.7134142Z [CI_JOB_NAME=mingw-check]
2020-04-03T22:08:38.7449278Z [CI_JOB_NAME=mingw-check]
2020-04-03T22:08:38.7513549Z == clock drift check ==
2020-04-03T22:08:38.7513860Z   local time: Fri Apr  3 22:08:38 UTC 2020
2020-04-03T22:08:38.9196505Z   network time: Fri, 03 Apr 2020 22:08:38 GMT
2020-04-03T22:08:38.9227181Z Starting sccache server...
2020-04-03T22:08:39.0040160Z configure: processing command line
2020-04-03T22:08:39.0040796Z configure: 
2020-04-03T22:08:39.0041857Z configure: rust.parallel-compiler := True
---
2020-04-03T22:10:01.1602409Z     Checking core v0.0.0 (/checkout/src/libcore)
2020-04-03T22:10:07.6333596Z error[E0599]: no method named `clone` found for type parameter `T` in the current scope
2020-04-03T22:10:07.6334771Z     --> src/libcore/slice/mod.rs:2160:25
2020-04-03T22:10:07.6335519Z      |
2020-04-03T22:10:07.6336182Z 2160 |             *el = value.clone();
2020-04-03T22:10:07.6337006Z      |                         ^^^^^ method not found in `T`
2020-04-03T22:10:07.6338445Z      = help: items from traits can only be used if the type parameter is bounded by the trait
2020-04-03T22:10:07.6339371Z help: the following trait defines an item `clone`, perhaps you need to restrict type parameter `T` with it:
2020-04-03T22:10:07.6340078Z      |
2020-04-03T22:10:07.6340078Z      |
2020-04-03T22:10:07.6340733Z 58   | impl<T: clone::Clone> [T] {
2020-04-03T22:10:07.6341900Z 
2020-04-03T22:10:08.0321187Z    Compiling libc v0.2.66
2020-04-03T22:10:09.3649781Z    Compiling autocfg v0.1.7
2020-04-03T22:10:09.6400574Z error: aborting due to previous error
---
2020-04-03T22:10:10.3895577Z expected success, got: exit code: 101
2020-04-03T22:10:10.3907433Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-03T22:10:10.3907732Z Build completed unsuccessfully in 0:01:31
2020-04-03T22:10:10.3956643Z == clock drift check ==
2020-04-03T22:10:11.7515758Z   local time: Fri Apr  3 22:10:11 UTC 2020
2020-04-03T22:10:11.8976923Z   network time: Fri, 03 Apr 2020 22:10:11 GMT
2020-04-03T22:10:14.0214536Z 
2020-04-03T22:10:14.0214536Z 
2020-04-03T22:10:14.0275262Z ##[error]Bash exited with code '1'.
2020-04-03T22:10:14.0352480Z ##[section]Finishing: Run build
2020-04-03T22:10:14.0390594Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70752/merge to s
2020-04-03T22:10:14.0397548Z Task         : Get sources
2020-04-03T22:10:14.0398175Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-03T22:10:14.0398735Z Version      : 1.0.0
2020-04-03T22:10:14.0399124Z Author       : Microsoft
2020-04-03T22:10:14.0399124Z Author       : Microsoft
2020-04-03T22:10:14.0399758Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-03T22:10:14.0400469Z ==============================================================================
2020-04-03T22:10:14.3335064Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-03T22:10:14.3375510Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70752/merge to s
2020-04-03T22:10:14.3457814Z Cleaning up task key
2020-04-03T22:10:14.3458990Z Start cleaning up orphan processes.
2020-04-03T22:10:14.3642870Z Terminate orphan process: pid (5095) (python)
2020-04-03T22:10:14.3789497Z ##[section]Finishing: Finalize Job
