plain
2020-04-13T17:48:11.2058616Z ========================== Starting Command Output ===========================
2020-04-13T17:48:11.2061478Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e269a970-7dbb-4808-944c-bfb1660ade89.sh
2020-04-13T17:48:11.2061764Z 
2020-04-13T17:48:11.2066256Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-13T17:48:11.2087301Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70705/merge to s
2020-04-13T17:48:11.2090858Z Task         : Get sources
2020-04-13T17:48:11.2091185Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-13T17:48:11.2091506Z Version      : 1.0.0
2020-04-13T17:48:11.2091726Z Author       : Microsoft
---
2020-04-13T17:48:12.2246090Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-13T17:48:12.2253127Z ##[command]git config gc.auto 0
2020-04-13T17:48:12.2258013Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-13T17:48:12.2262371Z ##[command]git config --get-all http.proxy
2020-04-13T17:48:12.2270465Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70705/merge:refs/remotes/pull/70705/merge
---
2020-04-13T17:50:17.7208489Z E: Unable to fetch some archives, maybe run apt-get update or try with --fix-missing?
2020-04-13T17:50:17.7252833Z 
2020-04-13T17:50:17.7322686Z ##[error]Bash exited with code '100'.
2020-04-13T17:50:17.7339408Z ##[section]Finishing: Install awscli
2020-04-13T17:50:17.7393301Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70705/merge to s
2020-04-13T17:50:17.7398042Z Task         : Get sources
2020-04-13T17:50:17.7398442Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-13T17:50:17.7398702Z Version      : 1.0.0
2020-04-13T17:50:17.7398906Z Author       : Microsoft
2020-04-13T17:50:17.7398906Z Author       : Microsoft
2020-04-13T17:50:17.7399202Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-13T17:50:17.7399530Z ==============================================================================
2020-04-13T17:50:18.0540971Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-13T17:50:18.0587470Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70705/merge to s
2020-04-13T17:50:18.0683216Z Cleaning up task key
2020-04-13T17:50:18.0684492Z Start cleaning up orphan processes.
2020-04-13T17:50:18.0860425Z Terminate orphan process: pid (3643) (python)
2020-04-13T17:50:18.0974292Z ##[section]Finishing: Finalize Job
