plain
2020-02-13T02:49:21.8880740Z ========================== Starting Command Output ===========================
2020-02-13T02:49:21.8883533Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/aadd5563-fdbc-48be-8087-79dec5b23467.sh
2020-02-13T02:49:21.8883578Z 
2020-02-13T02:49:21.8886443Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-13T02:49:21.8893195Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69119/merge to s
2020-02-13T02:49:21.8894980Z Task         : Get sources
2020-02-13T02:49:21.8895013Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-13T02:49:21.8895046Z Version      : 1.0.0
2020-02-13T02:49:21.8895122Z Author       : Microsoft
---
2020-02-13T02:49:22.9434513Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-13T02:49:22.9450532Z ##[command]git config gc.auto 0
2020-02-13T02:49:22.9459485Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-13T02:49:22.9466137Z ##[command]git config --get-all http.proxy
2020-02-13T02:49:22.9480316Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69119/merge:refs/remotes/pull/69119/merge
---
2020-02-13T03:23:29.9045629Z   local time: Thu Feb 13 03:23:29 UTC 2020
2020-02-13T03:23:30.0075180Z   network time: Thu, 13 Feb 2020 03:23:30 GMT
2020-02-13T03:23:30.0078458Z == end clock drift check ==
2020-02-13T03:23:30.7168181Z 
2020-02-13T03:23:30.7275155Z ##[error]Bash exited with code '1'.
2020-02-13T03:23:30.7288004Z ##[section]Finishing: Run build
2020-02-13T03:23:30.7307282Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69119/merge to s
2020-02-13T03:23:30.7309172Z Task         : Get sources
2020-02-13T03:23:30.7309219Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-13T03:23:30.7309282Z Version      : 1.0.0
2020-02-13T03:23:30.7309323Z Author       : Microsoft
2020-02-13T03:23:30.7309323Z Author       : Microsoft
2020-02-13T03:23:30.7309368Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-13T03:23:30.7309435Z ==============================================================================
2020-02-13T03:23:31.1721459Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-13T03:23:31.1759827Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69119/merge to s
2020-02-13T03:23:31.1872319Z Cleaning up task key
2020-02-13T03:23:31.1873337Z Start cleaning up orphan processes.
2020-02-13T03:23:31.1985863Z Terminate orphan process: pid (3865) (python)
2020-02-13T03:23:31.2210135Z ##[section]Finishing: Finalize Job
