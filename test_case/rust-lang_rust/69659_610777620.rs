plain
2020-04-08T06:31:33.7134874Z ========================== Starting Command Output ===========================
2020-04-08T06:31:33.7137375Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/7358b12f-543d-4c48-a5db-19d587b16949.sh
2020-04-08T06:31:33.7137670Z 
2020-04-08T06:31:33.7141457Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-08T06:31:33.7161069Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69659/merge to s
2020-04-08T06:31:33.7164317Z Task         : Get sources
2020-04-08T06:31:33.7164628Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-08T06:31:33.7164926Z Version      : 1.0.0
2020-04-08T06:31:33.7165143Z Author       : Microsoft
---
2020-04-08T06:31:34.7358679Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-08T06:31:34.7401938Z ##[command]git config gc.auto 0
2020-04-08T06:31:34.7448401Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-08T06:31:34.7479998Z ##[command]git config --get-all http.proxy
2020-04-08T06:31:34.7585685Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69659/merge:refs/remotes/pull/69659/merge
---
2020-04-08T06:32:38.6182530Z E: Unable to fetch some archives, maybe run apt-get update or try with --fix-missing?
2020-04-08T06:32:38.6217912Z 
2020-04-08T06:32:38.6283377Z ##[error]Bash exited with code '100'.
2020-04-08T06:32:38.6296779Z ##[section]Finishing: Install awscli
2020-04-08T06:32:38.6362844Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69659/merge to s
2020-04-08T06:32:38.6367716Z Task         : Get sources
2020-04-08T06:32:38.6368098Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-08T06:32:38.6368428Z Version      : 1.0.0
2020-04-08T06:32:38.6368790Z Author       : Microsoft
2020-04-08T06:32:38.6368790Z Author       : Microsoft
2020-04-08T06:32:38.6369171Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-08T06:32:38.6369598Z ==============================================================================
2020-04-08T06:32:38.9484331Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-08T06:32:38.9527494Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69659/merge to s
2020-04-08T06:32:38.9612157Z Cleaning up task key
2020-04-08T06:32:38.9613484Z Start cleaning up orphan processes.
2020-04-08T06:32:38.9794438Z Terminate orphan process: pid (3748) (python)
2020-04-08T06:32:38.9909725Z ##[section]Finishing: Finalize Job
