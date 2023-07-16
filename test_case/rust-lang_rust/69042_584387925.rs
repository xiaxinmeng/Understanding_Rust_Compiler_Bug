plain
2020-02-10T22:21:20.3979443Z ========================== Starting Command Output ===========================
2020-02-10T22:21:20.3980768Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4b4806fb-b198-4868-a2cb-b39f09cf0681.sh
2020-02-10T22:21:20.3980836Z 
2020-02-10T22:21:20.3983801Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-10T22:21:20.3991202Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69042/merge to s
2020-02-10T22:21:20.3992910Z Task         : Get sources
2020-02-10T22:21:20.3992983Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-10T22:21:20.3993019Z Version      : 1.0.0
2020-02-10T22:21:20.3993049Z Author       : Microsoft
---
2020-02-10T22:21:21.1471504Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-10T22:21:21.1541616Z ##[command]git config gc.auto 0
2020-02-10T22:21:21.1611743Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-10T22:21:21.1662744Z ##[command]git config --get-all http.proxy
2020-02-10T22:21:21.1797741Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69042/merge:refs/remotes/pull/69042/merge
---
2020-02-10T22:23:47.3118640Z #########################################################                 80.5%
2020-02-10T22:23:47.3118848Z ######################################################################## 100.0%
2020-02-10T22:23:47.5593561Z extracting /checkout/obj/build/cache/2020-01-31/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz
2020-02-10T22:23:47.6321772Z     Updating crates.io index
2020-02-10T22:24:00.9539316Z error: failed to select a version for the requirement `backtrace = "^0.3.44"`
2020-02-10T22:24:00.9541423Z   candidate versions found which didn't match: 0.3.43, 0.3.41, 0.3.40, ...
2020-02-10T22:24:00.9542189Z   location searched: crates.io index
2020-02-10T22:24:00.9542584Z required by package `std v0.0.0 (/checkout/src/libstd)`
2020-02-10T22:24:00.9564291Z Build completed unsuccessfully in 0:00:25
2020-02-10T22:24:00.9611298Z == clock drift check ==
2020-02-10T22:24:00.9621262Z   local time: Mon Feb 10 22:24:00 UTC 2020
2020-02-10T22:24:01.2461295Z   network time: Mon, 10 Feb 2020 22:24:01 GMT
2020-02-10T22:24:01.2461295Z   network time: Mon, 10 Feb 2020 22:24:01 GMT
2020-02-10T22:24:01.2464229Z == end clock drift check ==
2020-02-10T22:24:08.9195044Z 
2020-02-10T22:24:08.9277927Z ##[error]Bash exited with code '1'.
2020-02-10T22:24:08.9293347Z ##[section]Finishing: Run build
2020-02-10T22:24:08.9312646Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69042/merge to s
2020-02-10T22:24:08.9315778Z Task         : Get sources
2020-02-10T22:24:08.9315825Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-10T22:24:08.9315887Z Version      : 1.0.0
2020-02-10T22:24:08.9315929Z Author       : Microsoft
2020-02-10T22:24:08.9315929Z Author       : Microsoft
2020-02-10T22:24:08.9315976Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-10T22:24:08.9316065Z ==============================================================================
2020-02-10T22:24:09.2805701Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-10T22:24:09.2845864Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69042/merge to s
2020-02-10T22:24:09.2940407Z Cleaning up task key
2020-02-10T22:24:09.2941278Z Start cleaning up orphan processes.
2020-02-10T22:24:09.3031443Z Terminate orphan process: pid (3997) (python)
2020-02-10T22:24:09.3188399Z ##[section]Finishing: Finalize Job
