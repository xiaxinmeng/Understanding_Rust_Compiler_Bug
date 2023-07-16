plain
2019-09-25T23:35:53.4466112Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-25T23:35:53.4707858Z ##[command]git config gc.auto 0
2019-09-25T23:35:53.4758198Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-25T23:35:53.4815685Z ##[command]git config --get-all http.proxy
2019-09-25T23:35:53.4966176Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64790/merge:refs/remotes/pull/64790/merge
---
2019-09-26T00:24:03.9841780Z == clock drift check ==
2019-09-26T00:24:03.9859645Z   local time: Thu Sep 26 00:24:03 UTC 2019
2019-09-26T00:24:04.0401149Z   network time: Thu, 26 Sep 2019 00:24:04 GMT
2019-09-26T00:24:04.0404600Z == end clock drift check ==
2019-09-26T00:24:07.1678341Z ##[error]Bash exited with code '1'.
2019-09-26T00:24:07.1722661Z ##[section]Starting: Checkout
2019-09-26T00:24:07.1724900Z ==============================================================================
2019-09-26T00:24:07.1724949Z Task         : Get sources
2019-09-26T00:24:07.1725157Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
