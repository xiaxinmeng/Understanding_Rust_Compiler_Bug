plain
2020-03-24T23:01:48.2053703Z ========================== Starting Command Output ===========================
2020-03-24T23:01:48.2136552Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/5e55115d-ba0b-4f44-9e66-cbdc5b9aa634.sh
2020-03-24T23:01:48.2137138Z 
2020-03-24T23:01:48.2167460Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-24T23:01:48.2195454Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-03-24T23:01:48.2199152Z Task         : Get sources
2020-03-24T23:01:48.2199475Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-24T23:01:48.2199782Z Version      : 1.0.0
2020-03-24T23:01:48.2199993Z Author       : Microsoft
---
2020-03-24T23:01:49.2018514Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-24T23:01:49.2027836Z ##[command]git config gc.auto 0
2020-03-24T23:01:49.2032679Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-24T23:01:49.2036423Z ##[command]git config --get-all http.proxy
2020-03-24T23:01:49.2045614Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69406/merge:refs/remotes/pull/69406/merge
---
2020-03-24T23:04:25.6850576Z  ---> 3fc1b512c57b
2020-03-24T23:04:25.6850825Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-03-24T23:04:25.6851193Z  ---> Using cache
2020-03-24T23:04:25.6851533Z  ---> 5ee4295733f4
2020-03-24T23:04:25.6852914Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-03-24T23:04:25.6854499Z  ---> 3d07a0fa42fe
2020-03-24T23:04:25.6894395Z Successfully built 3d07a0fa42fe
2020-03-24T23:04:25.6943904Z Successfully tagged rust-ci:latest
2020-03-24T23:04:25.7259964Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
---
2020-03-24T23:04:49.3510438Z   local time: Tue Mar 24 23:04:49 UTC 2020
2020-03-24T23:04:49.3948806Z   network time: Tue, 24 Mar 2020 23:04:49 GMT
2020-03-24T23:04:49.3953111Z == end clock drift check ==
2020-03-24T23:04:57.1217929Z 
2020-03-24T23:04:57.1310986Z ##[error]Bash exited with code '1'.
2020-03-24T23:04:57.1327650Z ##[section]Finishing: Run build
2020-03-24T23:04:57.1378032Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-03-24T23:04:57.1383353Z Task         : Get sources
2020-03-24T23:04:57.1383741Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-24T23:04:57.1384111Z Version      : 1.0.0
2020-03-24T23:04:57.1384362Z Author       : Microsoft
2020-03-24T23:04:57.1384362Z Author       : Microsoft
2020-03-24T23:04:57.1384799Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-24T23:04:57.1385278Z ==============================================================================
2020-03-24T23:04:57.5227824Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-24T23:04:57.5271920Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-03-24T23:04:57.5374576Z Cleaning up task key
2020-03-24T23:04:57.5376228Z Start cleaning up orphan processes.
2020-03-24T23:04:57.5615402Z Terminate orphan process: pid (3368) (python)
2020-03-24T23:04:57.5754071Z ##[section]Finishing: Finalize Job
