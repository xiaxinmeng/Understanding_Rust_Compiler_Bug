plain
2020-01-29T12:42:01.0953297Z ========================== Starting Command Output ===========================
2020-01-29T12:42:01.0955171Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ca148837-195a-4f9d-988b-dec431d07497.sh
2020-01-29T12:42:01.0955358Z 
2020-01-29T12:42:01.0958410Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-29T12:42:01.0963754Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68639/merge to s
2020-01-29T12:42:01.0965227Z Task         : Get sources
2020-01-29T12:42:01.0965303Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-29T12:42:01.0965336Z Version      : 1.0.0
2020-01-29T12:42:01.0965369Z Author       : Microsoft
---
2020-01-29T12:42:01.8796952Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-29T12:42:01.8805865Z ##[command]git config gc.auto 0
2020-01-29T12:42:01.8807995Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-29T12:42:01.8810440Z ##[command]git config --get-all http.proxy
2020-01-29T12:42:01.8816029Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68639/merge:refs/remotes/pull/68639/merge
