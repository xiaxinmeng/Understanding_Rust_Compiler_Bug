plain
2020-03-10T10:59:33.6375270Z ========================== Starting Command Output ===========================
2020-03-10T10:59:33.6380701Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c04e70a7-0679-4035-bbec-3a3d41c60c5d.sh
2020-03-10T10:59:33.6381249Z 
2020-03-10T10:59:33.6385433Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-10T10:59:33.6407636Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69766/merge to s
2020-03-10T10:59:33.6412928Z Task         : Get sources
2020-03-10T10:59:33.6413542Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-10T10:59:33.6413840Z Version      : 1.0.0
2020-03-10T10:59:33.6414033Z Author       : Microsoft
---
2020-03-10T10:59:36.4868952Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-10T10:59:36.4880706Z ##[command]git config gc.auto 0
2020-03-10T10:59:36.4889138Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-10T10:59:36.4896673Z ##[command]git config --get-all http.proxy
2020-03-10T10:59:36.4908383Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69766/merge:refs/remotes/pull/69766/merge
---
2020-03-10T11:05:19.9528620Z    |
2020-03-10T11:05:19.9529184Z 46 | //! struct Point {
2020-03-10T11:05:19.9529812Z    | ^^^^^^^^^^^^^^^^^^
2020-03-10T11:05:19.9530286Z    |
2020-03-10T11:05:19.9531027Z    = note: inner doc comments like this (starting with `//!` or `/*!`) can only appear before items
2020-03-10T11:05:19.9537359Z error: aborting due to previous error
2020-03-10T11:05:19.9537613Z 
2020-03-10T11:05:19.9566643Z error: could not compile `core`.
2020-03-10T11:05:19.9575137Z warning: build failed, waiting for other jobs to finish...
---
2020-03-10T11:05:24.6932218Z   local time: Tue Mar 10 11:05:24 UTC 2020
2020-03-10T11:05:24.7851730Z   network time: Tue, 10 Mar 2020 11:05:24 GMT
2020-03-10T11:05:24.7852150Z == end clock drift check ==
2020-03-10T11:05:33.1545602Z 
2020-03-10T11:05:33.1630347Z ##[error]Bash exited with code '1'.
2020-03-10T11:05:33.1648617Z ##[section]Finishing: Run build
2020-03-10T11:05:33.1697408Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69766/merge to s
2020-03-10T11:05:33.1702686Z Task         : Get sources
2020-03-10T11:05:33.1703095Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-10T11:05:33.1703480Z Version      : 1.0.0
2020-03-10T11:05:33.1703734Z Author       : Microsoft
2020-03-10T11:05:33.1703734Z Author       : Microsoft
2020-03-10T11:05:33.1704149Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-10T11:05:33.1704647Z ==============================================================================
2020-03-10T11:05:33.5001438Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-10T11:05:33.5050303Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69766/merge to s
2020-03-10T11:05:33.5146396Z Cleaning up task key
2020-03-10T11:05:33.5147918Z Start cleaning up orphan processes.
2020-03-10T11:05:33.5344480Z Terminate orphan process: pid (4362) (python)
2020-03-10T11:05:33.5515276Z ##[section]Finishing: Finalize Job
