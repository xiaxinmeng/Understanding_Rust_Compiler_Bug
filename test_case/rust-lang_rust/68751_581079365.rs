plain
2020-02-01T23:06:41.6634304Z ========================== Starting Command Output ===========================
2020-02-01T23:06:41.6637082Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/26880c7c-43f6-4d1b-9f93-b6524a059dd9.sh
2020-02-01T23:06:41.6637287Z 
2020-02-01T23:06:41.6882050Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-01T23:06:41.6888017Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68751/merge to s
2020-02-01T23:06:41.6890315Z Task         : Get sources
2020-02-01T23:06:41.6890349Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-01T23:06:41.6890381Z Version      : 1.0.0
2020-02-01T23:06:41.6890428Z Author       : Microsoft
---
2020-02-01T23:06:42.6594411Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-01T23:06:42.6605266Z ##[command]git config gc.auto 0
2020-02-01T23:06:42.6608108Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-01T23:06:42.6610707Z ##[command]git config --get-all http.proxy
2020-02-01T23:06:42.6679496Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68751/merge:refs/remotes/pull/68751/merge
---
2020-02-01T23:12:11.0039907Z   local time: Sat Feb  1 23:12:11 UTC 2020
2020-02-01T23:12:11.5455454Z   network time: Sat, 01 Feb 2020 23:12:11 GMT
2020-02-01T23:12:11.5459506Z == end clock drift check ==
2020-02-01T23:12:12.4935974Z 
2020-02-01T23:12:12.5020775Z ##[error]Bash exited with code '1'.
2020-02-01T23:12:12.5035904Z ##[section]Finishing: Run build
2020-02-01T23:12:12.5053275Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68751/merge to s
2020-02-01T23:12:12.5055395Z Task         : Get sources
2020-02-01T23:12:12.5055446Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-01T23:12:12.5055516Z Version      : 1.0.0
2020-02-01T23:12:12.5055563Z Author       : Microsoft
2020-02-01T23:12:12.5055563Z Author       : Microsoft
2020-02-01T23:12:12.5055613Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-01T23:12:12.5055685Z ==============================================================================
2020-02-01T23:12:12.9542101Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-01T23:12:12.9588633Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68751/merge to s
2020-02-01T23:12:12.9713078Z Cleaning up task key
2020-02-01T23:12:12.9713948Z Start cleaning up orphan processes.
2020-02-01T23:12:12.9839614Z Terminate orphan process: pid (6326) (python)
2020-02-01T23:12:13.0048450Z ##[section]Finishing: Finalize Job
