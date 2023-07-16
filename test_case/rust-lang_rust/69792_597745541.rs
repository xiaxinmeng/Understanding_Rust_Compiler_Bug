plain
2020-03-11T16:32:46.0800403Z ========================== Starting Command Output ===========================
2020-03-11T16:32:46.0804545Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b8cb0517-4ea3-42c2-b481-e19e00785563.sh
2020-03-11T16:32:46.0805076Z 
2020-03-11T16:32:46.0808922Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-11T16:32:46.0827722Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69792/merge to s
2020-03-11T16:32:46.0831004Z Task         : Get sources
2020-03-11T16:32:46.0831248Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-11T16:32:46.0831530Z Version      : 1.0.0
2020-03-11T16:32:46.0831851Z Author       : Microsoft
---
2020-03-11T16:32:47.3241867Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-11T16:32:47.3248187Z ##[command]git config gc.auto 0
2020-03-11T16:32:47.3251124Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-11T16:32:47.3254999Z ##[command]git config --get-all http.proxy
2020-03-11T16:32:47.3262404Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69792/merge:refs/remotes/pull/69792/merge
