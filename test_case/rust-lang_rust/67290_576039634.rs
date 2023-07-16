plain
2020-01-19T19:31:08.9563979Z ========================== Starting Command Output ===========================
2020-01-19T19:31:08.9565475Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9f4e992f-892f-4f96-a74e-ca851a57db42.sh
2020-01-19T19:31:08.9565508Z 
2020-01-19T19:31:08.9567941Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-19T19:31:08.9573874Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67290/merge to s
2020-01-19T19:31:08.9575504Z Task         : Get sources
2020-01-19T19:31:08.9575536Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-19T19:31:08.9575568Z Version      : 1.0.0
2020-01-19T19:31:08.9575601Z Author       : Microsoft
---
2020-01-19T19:31:09.9569875Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-19T19:31:09.9581477Z ##[command]git config gc.auto 0
2020-01-19T19:31:09.9583779Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-19T19:31:09.9585690Z ##[command]git config --get-all http.proxy
2020-01-19T19:31:09.9592121Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67290/merge:refs/remotes/pull/67290/merge
