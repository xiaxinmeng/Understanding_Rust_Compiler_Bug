plain
2020-01-17T15:14:04.3017986Z ========================== Starting Command Output ===========================
2020-01-17T15:14:04.3019483Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/74818a25-f97e-48be-8336-fa57410b8897.sh
2020-01-17T15:14:04.3019522Z 
2020-01-17T15:14:04.3077871Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-17T15:14:04.3082947Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68313/merge to s
2020-01-17T15:14:04.3084459Z Task         : Get sources
2020-01-17T15:14:04.3084488Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-17T15:14:04.3084516Z Version      : 1.0.0
2020-01-17T15:14:04.3084542Z Author       : Microsoft
---
2020-01-17T15:14:05.3587109Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-17T15:14:05.3604047Z ##[command]git config gc.auto 0
2020-01-17T15:14:05.3608065Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-17T15:14:05.3610193Z ##[command]git config --get-all http.proxy
2020-01-17T15:14:05.3628068Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68313/merge:refs/remotes/pull/68313/merge
