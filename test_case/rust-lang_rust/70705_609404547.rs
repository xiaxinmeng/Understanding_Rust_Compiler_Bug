plain
2020-04-05T11:52:10.1554387Z ========================== Starting Command Output ===========================
2020-04-05T11:52:10.1556620Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c8c3be85-c073-428f-837d-ce228b68ea55.sh
2020-04-05T11:52:10.1557305Z 
2020-04-05T11:52:10.1561368Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-05T11:52:10.1579299Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70705/merge to s
2020-04-05T11:52:10.1582297Z Task         : Get sources
2020-04-05T11:52:10.1582583Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-05T11:52:10.1582861Z Version      : 1.0.0
2020-04-05T11:52:10.1583047Z Author       : Microsoft
---
2020-04-05T11:52:11.1553234Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-05T11:52:11.1557045Z ##[command]git config gc.auto 0
2020-04-05T11:52:11.1559480Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-05T11:52:11.1561725Z ##[command]git config --get-all http.proxy
2020-04-05T11:52:11.1565933Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70705/merge:refs/remotes/pull/70705/merge
---
2020-04-05T11:54:16.0257091Z  ---> 3fc1b512c57b
2020-04-05T11:54:16.0257433Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-05T11:54:16.0257896Z  ---> Using cache
2020-04-05T11:54:16.0287683Z  ---> 5ee4295733f4
2020-04-05T11:54:16.0290254Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-05T11:54:16.0291682Z  ---> 3d07a0fa42fe
2020-04-05T11:54:16.0291958Z Successfully built 3d07a0fa42fe
2020-04-05T11:54:16.0293430Z Successfully tagged rust-ci:latest
2020-04-05T11:54:16.0518277Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-05T11:54:16.0518277Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-05T11:54:16.0532896Z Looks like docker image is the same as before, not uploading
2020-04-05T11:54:22.7528246Z [CI_JOB_NAME=mingw-check]
2020-04-05T11:54:22.7761669Z [CI_JOB_NAME=mingw-check]
2020-04-05T11:54:22.7785291Z == clock drift check ==
2020-04-05T11:54:22.7792609Z   local time: Sun Apr  5 11:54:22 UTC 2020
2020-04-05T11:54:23.0665953Z   network time: Sun, 05 Apr 2020 11:54:23 GMT
2020-04-05T11:54:23.0682804Z Starting sccache server...
2020-04-05T11:54:23.1425434Z configure: processing command line
2020-04-05T11:54:23.1425657Z configure: 
2020-04-05T11:54:23.1426364Z configure: rust.parallel-compiler := True
---
2020-04-05T11:55:42.4989655Z expected success, got: exit code: 101
2020-04-05T11:55:42.5000302Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-05T11:55:42.5000584Z Build completed unsuccessfully in 0:01:19
2020-04-05T11:55:42.5043282Z == clock drift check ==
2020-04-05T11:55:42.5058112Z   local time: Sun Apr  5 11:55:42 UTC 2020
2020-04-05T11:55:42.5704349Z   network time: Sun, 05 Apr 2020 11:55:42 GMT
2020-04-05T11:55:43.8086911Z 
2020-04-05T11:55:43.8086911Z 
2020-04-05T11:55:43.8139528Z ##[error]Bash exited with code '1'.
2020-04-05T11:55:43.8149117Z ##[section]Finishing: Run build
2020-04-05T11:55:43.8182363Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70705/merge to s
2020-04-05T11:55:43.8187157Z Task         : Get sources
2020-04-05T11:55:43.8187529Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-05T11:55:43.8187862Z Version      : 1.0.0
2020-04-05T11:55:43.8188092Z Author       : Microsoft
2020-04-05T11:55:43.8188092Z Author       : Microsoft
2020-04-05T11:55:43.8188466Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-05T11:55:43.8188978Z ==============================================================================
2020-04-05T11:55:44.0719778Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-05T11:55:44.0759617Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70705/merge to s
2020-04-05T11:55:44.0822514Z Cleaning up task key
2020-04-05T11:55:44.0823426Z Start cleaning up orphan processes.
2020-04-05T11:55:44.0966341Z Terminate orphan process: pid (3768) (python)
2020-04-05T11:55:44.1130791Z ##[section]Finishing: Finalize Job
