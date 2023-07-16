plain
2020-02-05T03:34:37.1546778Z ========================== Starting Command Output ===========================
2020-02-05T03:34:37.1548680Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d7449a3e-d8fd-4114-a028-1835ceda1138.sh
2020-02-05T03:34:37.1548732Z 
2020-02-05T03:34:37.1554582Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-05T03:34:37.1562220Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68845/merge to s
2020-02-05T03:34:37.1564245Z Task         : Get sources
2020-02-05T03:34:37.1564283Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-05T03:34:37.1564321Z Version      : 1.0.0
2020-02-05T03:34:37.1564410Z Author       : Microsoft
---
2020-02-05T03:34:38.2311427Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-05T03:34:38.2323772Z ##[command]git config gc.auto 0
2020-02-05T03:34:38.2326553Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-05T03:34:38.2328882Z ##[command]git config --get-all http.proxy
2020-02-05T03:34:38.2335678Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68845/merge:refs/remotes/pull/68845/merge
