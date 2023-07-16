plain
2020-02-06T06:47:02.5990878Z ========================== Starting Command Output ===========================
2020-02-06T06:47:02.5993371Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/39044897-ea3c-42df-85b1-8069c748fe95.sh
2020-02-06T06:47:02.5993429Z 
2020-02-06T06:47:02.5997257Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-06T06:47:02.6005943Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68850/merge to s
2020-02-06T06:47:02.6008072Z Task         : Get sources
2020-02-06T06:47:02.6008110Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-06T06:47:02.6008827Z Version      : 1.0.0
2020-02-06T06:47:02.6008873Z Author       : Microsoft
---
2020-02-06T06:47:07.2638007Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-06T06:47:07.2982595Z ##[command]git config gc.auto 0
2020-02-06T06:47:07.3063598Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-06T06:47:07.3119423Z ##[command]git config --get-all http.proxy
2020-02-06T06:47:07.3271188Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68850/merge:refs/remotes/pull/68850/merge
---
2020-02-06T06:48:34.4309391Z E: Unable to fetch some archives, maybe run apt-get update or try with --fix-missing?
2020-02-06T06:48:34.4352687Z 
2020-02-06T06:48:34.4451815Z ##[error]Bash exited with code '100'.
2020-02-06T06:48:34.4463110Z ##[section]Finishing: Install awscli
2020-02-06T06:48:34.4484692Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68850/merge to s
2020-02-06T06:48:34.4489623Z Task         : Get sources
2020-02-06T06:48:34.4492693Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-06T06:48:34.4492915Z Version      : 1.0.0
2020-02-06T06:48:34.4492956Z Author       : Microsoft
2020-02-06T06:48:34.4492956Z Author       : Microsoft
2020-02-06T06:48:34.4493003Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-06T06:48:34.4493099Z ==============================================================================
2020-02-06T06:48:34.8575869Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-06T06:48:34.8621962Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68850/merge to s
2020-02-06T06:48:34.8749670Z Cleaning up task key
2020-02-06T06:48:34.8750567Z Start cleaning up orphan processes.
2020-02-06T06:48:34.8875482Z Terminate orphan process: pid (4830) (python)
2020-02-06T06:48:34.9429177Z ##[section]Finishing: Finalize Job
