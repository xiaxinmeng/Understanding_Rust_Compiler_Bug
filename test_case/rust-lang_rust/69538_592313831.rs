plain
2020-02-28T04:36:59.1231483Z ========================== Starting Command Output ===========================
2020-02-28T04:36:59.1234394Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/dfd0400f-8155-4197-9010-b02c204fb2ad.sh
2020-02-28T04:36:59.1234663Z 
2020-02-28T04:36:59.1239897Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-28T04:36:59.1259330Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69538/merge to s
2020-02-28T04:36:59.1262907Z Task         : Get sources
2020-02-28T04:36:59.1263206Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-28T04:36:59.1263490Z Version      : 1.0.0
2020-02-28T04:36:59.1263684Z Author       : Microsoft
---
2020-02-28T04:37:00.3020847Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-28T04:37:00.3037024Z ##[command]git config gc.auto 0
2020-02-28T04:37:00.3041805Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-28T04:37:00.3045160Z ##[command]git config --get-all http.proxy
2020-02-28T04:37:00.3052413Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69538/merge:refs/remotes/pull/69538/merge
---
2020-02-28T04:40:26.2415990Z E: Unable to fetch some archives, maybe run apt-get update or try with --fix-missing?
2020-02-28T04:40:26.2457491Z 
2020-02-28T04:40:26.2531910Z ##[error]Bash exited with code '100'.
2020-02-28T04:40:26.2545776Z ##[section]Finishing: Install awscli
2020-02-28T04:40:26.2609573Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69538/merge to s
2020-02-28T04:40:26.2614626Z Task         : Get sources
2020-02-28T04:40:26.2614978Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-28T04:40:26.2615320Z Version      : 1.0.0
2020-02-28T04:40:26.2615544Z Author       : Microsoft
2020-02-28T04:40:26.2615544Z Author       : Microsoft
2020-02-28T04:40:26.2615903Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-28T04:40:26.2616335Z ==============================================================================
2020-02-28T04:40:26.6000575Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-28T04:40:26.6044664Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69538/merge to s
2020-02-28T04:40:26.6139380Z Cleaning up task key
2020-02-28T04:40:26.6141186Z Start cleaning up orphan processes.
2020-02-28T04:40:26.6342542Z Terminate orphan process: pid (4339) (python)
2020-02-28T04:40:26.6594820Z ##[section]Finishing: Finalize Job
