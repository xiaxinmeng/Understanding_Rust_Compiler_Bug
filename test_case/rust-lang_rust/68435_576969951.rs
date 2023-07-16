plain
2020-01-21T23:13:00.6781944Z ========================== Starting Command Output ===========================
2020-01-21T23:13:00.6784613Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b1444f29-6151-4d61-a6de-7ea54d19284b.sh
2020-01-21T23:13:00.6784650Z 
2020-01-21T23:13:00.6788411Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-21T23:13:00.6793449Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68435/merge to s
2020-01-21T23:13:00.6795331Z Task         : Get sources
2020-01-21T23:13:00.6795885Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-21T23:13:00.6795921Z Version      : 1.0.0
2020-01-21T23:13:00.6795953Z Author       : Microsoft
---
2020-01-21T23:13:01.5752891Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-21T23:13:01.5842508Z ##[command]git config gc.auto 0
2020-01-21T23:13:01.5917367Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-21T23:13:01.5989092Z ##[command]git config --get-all http.proxy
2020-01-21T23:13:01.6133925Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68435/merge:refs/remotes/pull/68435/merge
---
2020-01-22T01:45:50.4752445Z Verifying status of clippy-driver...
2020-01-22T01:45:50.4752796Z Verifying status of miri...
2020-01-22T01:45:50.4753291Z Verifying status of embedded-book...
2020-01-22T01:45:50.4753666Z Verifying status of rustc-guide...
2020-01-22T01:45:50.4754224Z error: Tool `clippy-driver` should be test-pass but is build-fail during beta week.
2020-01-22T01:45:50.4758493Z Build completed unsuccessfully in 0:00:01
2020-01-22T01:45:50.4813279Z == clock drift check ==
2020-01-22T01:45:50.4825862Z   local time: Wed Jan 22 01:45:50 UTC 2020
2020-01-22T01:45:51.0399637Z   network time: Wed, 22 Jan 2020 01:45:51 GMT
2020-01-22T01:45:51.0399637Z   network time: Wed, 22 Jan 2020 01:45:51 GMT
2020-01-22T01:45:51.0402218Z == end clock drift check ==
2020-01-22T01:45:51.3743772Z 
2020-01-22T01:45:51.3852176Z ##[error]Bash exited with code '1'.
2020-01-22T01:45:51.3864377Z ##[section]Finishing: Run build
2020-01-22T01:45:51.3885127Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68435/merge to s
2020-01-22T01:45:51.3887757Z Task         : Get sources
2020-01-22T01:45:51.3887805Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-22T01:45:51.3887852Z Version      : 1.0.0
2020-01-22T01:45:51.3887915Z Author       : Microsoft
2020-01-22T01:45:51.3887915Z Author       : Microsoft
2020-01-22T01:45:51.3887962Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-22T01:45:51.3888012Z ==============================================================================
2020-01-22T01:45:51.8335128Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-22T01:45:51.8380559Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68435/merge to s
2020-01-22T01:45:51.8506520Z Cleaning up task key
2020-01-22T01:45:51.8507869Z Start cleaning up orphan processes.
2020-01-22T01:45:51.8630264Z Terminate orphan process: pid (4095) (python)
2020-01-22T01:45:51.8898310Z ##[section]Finishing: Finalize Job
