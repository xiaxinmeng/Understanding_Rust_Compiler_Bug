plain
2020-03-25T17:46:43.0878218Z ========================== Starting Command Output ===========================
2020-03-25T17:46:43.0880806Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a71544a1-7773-4ac5-8442-fe2d5d50f91b.sh
2020-03-25T17:46:43.0881066Z 
2020-03-25T17:46:43.0885272Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-25T17:46:43.0904290Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70362/merge to s
2020-03-25T17:46:43.0907500Z Task         : Get sources
2020-03-25T17:46:43.0907792Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-25T17:46:43.0908074Z Version      : 1.0.0
2020-03-25T17:46:43.0908264Z Author       : Microsoft
---
2020-03-25T17:46:44.3634866Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-25T17:46:44.3647773Z ##[command]git config gc.auto 0
2020-03-25T17:46:44.3669255Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-25T17:46:44.3678330Z ##[command]git config --get-all http.proxy
2020-03-25T17:46:44.3693177Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70362/merge:refs/remotes/pull/70362/merge
---
2020-03-25T17:48:57.1223541Z  ---> 3fc1b512c57b
2020-03-25T17:48:57.1223755Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-03-25T17:48:57.1224122Z  ---> Using cache
2020-03-25T17:48:57.1224427Z  ---> 5ee4295733f4
2020-03-25T17:48:57.1225892Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-03-25T17:48:57.1227238Z  ---> 3d07a0fa42fe
2020-03-25T17:48:57.1276824Z Successfully built 3d07a0fa42fe
2020-03-25T17:48:57.1302363Z Successfully tagged rust-ci:latest
2020-03-25T17:48:57.1591319Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
---
2020-03-25T17:50:59.3979214Z   local time: Wed Mar 25 17:50:59 UTC 2020
2020-03-25T17:50:59.5600303Z   network time: Wed, 25 Mar 2020 17:50:59 GMT
2020-03-25T17:50:59.5600769Z == end clock drift check ==
2020-03-25T17:51:06.2018453Z 
2020-03-25T17:51:06.2057925Z ##[error]Bash exited with code '1'.
2020-03-25T17:51:06.2068696Z ##[section]Finishing: Run build
2020-03-25T17:51:06.2109266Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70362/merge to s
2020-03-25T17:51:06.2113901Z Task         : Get sources
2020-03-25T17:51:06.2114176Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-25T17:51:06.2114438Z Version      : 1.0.0
2020-03-25T17:51:06.2114629Z Author       : Microsoft
2020-03-25T17:51:06.2114629Z Author       : Microsoft
2020-03-25T17:51:06.2114911Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-25T17:51:06.2115234Z ==============================================================================
2020-03-25T17:51:06.5234827Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-25T17:51:06.5283581Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70362/merge to s
2020-03-25T17:51:06.5383425Z Cleaning up task key
2020-03-25T17:51:06.5384608Z Start cleaning up orphan processes.
2020-03-25T17:51:06.5563871Z Terminate orphan process: pid (3465) (python)
2020-03-25T17:51:06.5752132Z ##[section]Finishing: Finalize Job
