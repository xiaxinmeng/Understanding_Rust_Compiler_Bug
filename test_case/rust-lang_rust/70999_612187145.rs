plain
2020-04-10T19:37:16.2415466Z ========================== Starting Command Output ===========================
2020-04-10T19:37:16.2418873Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/daa4ae0f-40ed-474d-808d-866003dcdc66.sh
2020-04-10T19:37:16.2419248Z 
2020-04-10T19:37:16.2423187Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-10T19:37:16.2440084Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70999/merge to s
2020-04-10T19:37:16.2443740Z Task         : Get sources
2020-04-10T19:37:16.2444070Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-10T19:37:16.2444374Z Version      : 1.0.0
2020-04-10T19:37:16.2444580Z Author       : Microsoft
---
2020-04-10T19:37:17.5308353Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-10T19:37:17.5314866Z ##[command]git config gc.auto 0
2020-04-10T19:37:17.5318996Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-10T19:37:17.5322839Z ##[command]git config --get-all http.proxy
2020-04-10T19:37:17.5330911Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70999/merge:refs/remotes/pull/70999/merge
---
2020-04-10T19:39:58.9297512Z  ---> 78ad2f4d4aca
2020-04-10T19:39:58.9298631Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-10T19:39:58.9304994Z  ---> Using cache
2020-04-10T19:39:58.9305432Z  ---> 4d2dc61c4d00
2020-04-10T19:39:58.9306681Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-10T19:39:58.9312780Z  ---> 776b6266a8b7
2020-04-10T19:39:58.9407894Z Successfully built 776b6266a8b7
2020-04-10T19:39:59.5752142Z Successfully tagged rust-ci:latest
2020-04-10T19:39:59.5753590Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-10T19:39:59.5753590Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-10T19:39:59.5754618Z Looks like docker image is the same as before, not uploading
2020-04-10T19:40:06.1340752Z [CI_JOB_NAME=mingw-check]
2020-04-10T19:40:06.1574396Z [CI_JOB_NAME=mingw-check]
2020-04-10T19:40:06.1594411Z == clock drift check ==
2020-04-10T19:40:06.1600401Z   local time: Fri Apr 10 19:40:06 UTC 2020
2020-04-10T19:40:06.2828287Z   network time: Fri, 10 Apr 2020 19:40:06 GMT
2020-04-10T19:40:06.2853546Z Starting sccache server...
2020-04-10T19:40:06.3868068Z configure: processing command line
2020-04-10T19:40:06.3868831Z configure: 
2020-04-10T19:40:06.3870182Z configure: rust.parallel-compiler := True
---
2020-04-10T19:40:30.5916484Z   Read-only file system (os error 30)
2020-04-10T19:40:30.5937601Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
2020-04-10T19:40:30.5938275Z Build completed unsuccessfully in 0:00:24
2020-04-10T19:40:30.6035476Z == clock drift check ==
2020-04-10T19:40:31.5848850Z   local time: Fri Apr 10 19:40:30 UTC 2020
2020-04-10T19:40:31.5849980Z   network time: Fri, 10 Apr 2020 19:40:30 GMT
2020-04-10T19:40:38.4664066Z 
2020-04-10T19:40:38.4664066Z 
2020-04-10T19:40:38.4723637Z ##[error]Bash exited with code '1'.
2020-04-10T19:40:38.4736083Z ##[section]Finishing: Run build
2020-04-10T19:40:38.4771177Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70999/merge to s
2020-04-10T19:40:38.4775790Z Task         : Get sources
2020-04-10T19:40:38.4776078Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-10T19:40:38.4776351Z Version      : 1.0.0
2020-04-10T19:40:38.4776555Z Author       : Microsoft
2020-04-10T19:40:38.4776555Z Author       : Microsoft
2020-04-10T19:40:38.4776845Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-10T19:40:38.4777179Z ==============================================================================
2020-04-10T19:40:38.7428062Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-10T19:40:38.7464621Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70999/merge to s
2020-04-10T19:40:38.7541127Z Cleaning up task key
2020-04-10T19:40:38.7542343Z Start cleaning up orphan processes.
2020-04-10T19:40:38.7699080Z Terminate orphan process: pid (3436) (python)
2020-04-10T19:40:38.7798448Z ##[section]Finishing: Finalize Job
