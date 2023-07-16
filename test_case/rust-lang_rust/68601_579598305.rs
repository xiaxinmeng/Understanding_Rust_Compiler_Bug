plain
2020-01-29T05:10:33.1614511Z ========================== Starting Command Output ===========================
2020-01-29T05:10:33.1616885Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3af3b0f2-cc82-4923-a5dd-15f9e4ec31fb.sh
2020-01-29T05:10:33.1779110Z 
2020-01-29T05:10:33.1867734Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-29T05:10:33.1872228Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68601/merge to s
2020-01-29T05:10:33.1873394Z Task         : Get sources
2020-01-29T05:10:33.1873419Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-29T05:10:33.1873444Z Version      : 1.0.0
2020-01-29T05:10:33.1873501Z Author       : Microsoft
---
2020-01-29T05:10:34.0033709Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-29T05:10:34.0042021Z ##[command]git config gc.auto 0
2020-01-29T05:10:34.0043665Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-29T05:10:34.0045643Z ##[command]git config --get-all http.proxy
2020-01-29T05:10:34.0138283Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68601/merge:refs/remotes/pull/68601/merge
