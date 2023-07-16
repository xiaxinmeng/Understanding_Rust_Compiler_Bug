plain
2020-02-01T23:34:17.0799386Z ========================== Starting Command Output ===========================
2020-02-01T23:34:17.0801645Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/98d5b51a-cdc6-4a30-b9a1-c7d8e3a7f3f8.sh
2020-02-01T23:34:17.0801726Z 
2020-02-01T23:34:17.0805112Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-01T23:34:17.0811898Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68751/merge to s
2020-02-01T23:34:17.0813274Z Task         : Get sources
2020-02-01T23:34:17.0813302Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-01T23:34:17.0813329Z Version      : 1.0.0
2020-02-01T23:34:17.0813392Z Author       : Microsoft
---
2020-02-01T23:34:18.0589638Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-01T23:34:18.0601590Z ##[command]git config gc.auto 0
2020-02-01T23:34:18.0603420Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-01T23:34:18.0604928Z ##[command]git config --get-all http.proxy
2020-02-01T23:34:18.0612169Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68751/merge:refs/remotes/pull/68751/merge
---
2020-02-01T23:38:47.2069079Z   local time: Sat Feb  1 23:38:47 UTC 2020
2020-02-01T23:38:47.4960791Z   network time: Sat, 01 Feb 2020 23:38:47 GMT
2020-02-01T23:38:47.4964194Z == end clock drift check ==
2020-02-01T23:38:48.5030170Z 
2020-02-01T23:38:48.5164682Z ##[error]Bash exited with code '1'.
2020-02-01T23:38:48.5179032Z ##[section]Finishing: Run build
2020-02-01T23:38:48.5200144Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68751/merge to s
2020-02-01T23:38:48.5203256Z Task         : Get sources
2020-02-01T23:38:48.5203305Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-01T23:38:48.5203369Z Version      : 1.0.0
2020-02-01T23:38:48.5203432Z Author       : Microsoft
2020-02-01T23:38:48.5203432Z Author       : Microsoft
2020-02-01T23:38:48.5203479Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-01T23:38:48.5203548Z ==============================================================================
2020-02-01T23:38:48.9684880Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-01T23:38:48.9720191Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68751/merge to s
2020-02-01T23:38:48.9853427Z Cleaning up task key
2020-02-01T23:38:48.9854264Z Start cleaning up orphan processes.
2020-02-01T23:38:48.9975568Z Terminate orphan process: pid (3739) (python)
2020-02-01T23:38:49.0178658Z ##[section]Finishing: Finalize Job
