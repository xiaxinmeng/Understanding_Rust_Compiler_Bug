plain
2020-02-05T10:23:07.1462564Z ========================== Starting Command Output ===========================
2020-02-05T10:23:07.1465137Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d7b97bd8-8726-49f8-927b-56b8107ec0f2.sh
2020-02-05T10:23:07.1465310Z 
2020-02-05T10:23:07.1506916Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-05T10:23:07.1515299Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68859/merge to s
2020-02-05T10:23:07.1517107Z Task         : Get sources
2020-02-05T10:23:07.1517150Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-05T10:23:07.1517245Z Version      : 1.0.0
2020-02-05T10:23:07.1517289Z Author       : Microsoft
---
2020-02-05T10:23:08.1484234Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-05T10:23:08.1495959Z ##[command]git config gc.auto 0
2020-02-05T10:23:08.1498628Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-05T10:23:08.1500789Z ##[command]git config --get-all http.proxy
2020-02-05T10:23:08.1508104Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68859/merge:refs/remotes/pull/68859/merge
