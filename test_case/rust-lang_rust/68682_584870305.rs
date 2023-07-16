plain
2020-02-11T21:35:11.5898389Z ========================== Starting Command Output ===========================
2020-02-11T21:35:11.5899930Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/abe5f013-ffa5-459a-84fc-f28313f39746.sh
2020-02-11T21:35:11.5899964Z 
2020-02-11T21:35:11.5902841Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-11T21:35:11.5907645Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68682/merge to s
2020-02-11T21:35:11.5909337Z Task         : Get sources
2020-02-11T21:35:11.5909366Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-11T21:35:11.5909395Z Version      : 1.0.0
2020-02-11T21:35:11.5909597Z Author       : Microsoft
---
2020-02-11T21:35:12.4962817Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-11T21:35:12.5044668Z ##[command]git config gc.auto 0
2020-02-11T21:35:12.5119474Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-11T21:35:12.5193345Z ##[command]git config --get-all http.proxy
2020-02-11T21:35:12.5389681Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68682/merge:refs/remotes/pull/68682/merge
---
2020-02-11T21:41:24.4594219Z Found 0 error codes with no tests
2020-02-11T21:41:24.4594285Z Done!
2020-02-11T21:41:24.4594311Z 
2020-02-11T21:41:24.4594367Z 
2020-02-11T21:41:24.4595160Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-02-11T21:41:24.4595294Z 
2020-02-11T21:41:24.4595318Z 
2020-02-11T21:41:24.4595382Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-02-11T21:41:24.4595442Z Build completed unsuccessfully in 0:01:40
2020-02-11T21:41:24.4595442Z Build completed unsuccessfully in 0:01:40
2020-02-11T21:41:24.4641379Z == clock drift check ==
2020-02-11T21:41:24.4693402Z   local time: Tue Feb 11 21:41:24 UTC 2020
2020-02-11T21:41:24.6242038Z   network time: Tue, 11 Feb 2020 21:41:24 GMT
2020-02-11T21:41:24.6244039Z == end clock drift check ==
2020-02-11T21:41:25.3461686Z 
2020-02-11T21:41:25.3554429Z ##[error]Bash exited with code '1'.
2020-02-11T21:41:25.3565863Z ##[section]Finishing: Run build
2020-02-11T21:41:25.3580319Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68682/merge to s
2020-02-11T21:41:25.3582308Z Task         : Get sources
2020-02-11T21:41:25.3582367Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-11T21:41:25.3582557Z Version      : 1.0.0
2020-02-11T21:41:25.3582593Z Author       : Microsoft
2020-02-11T21:41:25.3582593Z Author       : Microsoft
2020-02-11T21:41:25.3582633Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-11T21:41:25.3582694Z ==============================================================================
2020-02-11T21:41:25.7497371Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-11T21:41:25.7542531Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68682/merge to s
2020-02-11T21:41:25.7655174Z Cleaning up task key
2020-02-11T21:41:25.7656113Z Start cleaning up orphan processes.
2020-02-11T21:41:25.7765902Z Terminate orphan process: pid (3575) (python)
2020-02-11T21:41:25.7941226Z ##[section]Finishing: Finalize Job
