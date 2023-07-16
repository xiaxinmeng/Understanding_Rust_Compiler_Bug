plain
2020-02-20T16:34:59.0315119Z ========================== Starting Command Output ===========================
2020-02-20T16:34:59.0318474Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c20d847c-2984-47b0-8b54-e0041327808d.sh
2020-02-20T16:34:59.0318819Z 
2020-02-20T16:34:59.0321993Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-20T16:34:59.0338400Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69317/merge to s
2020-02-20T16:34:59.0341464Z Task         : Get sources
2020-02-20T16:34:59.0341681Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-20T16:34:59.0341943Z Version      : 1.0.0
2020-02-20T16:34:59.0342087Z Author       : Microsoft
---
2020-02-20T16:35:00.0355217Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-20T16:35:00.0361013Z ##[command]git config gc.auto 0
2020-02-20T16:35:00.0365224Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-20T16:35:00.0368710Z ##[command]git config --get-all http.proxy
2020-02-20T16:35:00.0376422Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69317/merge:refs/remotes/pull/69317/merge
---
2020-02-20T16:38:29.4154180Z E: Unable to fetch some archives, maybe run apt-get update or try with --fix-missing?
2020-02-20T16:38:29.4188865Z 
2020-02-20T16:38:29.4252738Z ##[error]Bash exited with code '100'.
2020-02-20T16:38:29.4263694Z ##[section]Finishing: Install awscli
2020-02-20T16:38:29.4317325Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69317/merge to s
2020-02-20T16:38:29.4321735Z Task         : Get sources
2020-02-20T16:38:29.4322027Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-20T16:38:29.4322292Z Version      : 1.0.0
2020-02-20T16:38:29.4322465Z Author       : Microsoft
2020-02-20T16:38:29.4322465Z Author       : Microsoft
2020-02-20T16:38:29.4322745Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-20T16:38:29.4323085Z ==============================================================================
2020-02-20T16:38:29.7424985Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-20T16:38:29.7463759Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69317/merge to s
2020-02-20T16:38:29.7546364Z Cleaning up task key
2020-02-20T16:38:29.7547738Z Start cleaning up orphan processes.
2020-02-20T16:38:29.7727306Z Terminate orphan process: pid (3842) (python)
2020-02-20T16:38:29.7837783Z ##[section]Finishing: Finalize Job
