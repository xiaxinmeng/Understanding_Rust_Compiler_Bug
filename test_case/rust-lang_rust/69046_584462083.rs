plain
2020-02-11T02:55:31.8163312Z ========================== Starting Command Output ===========================
2020-02-11T02:55:31.8166032Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/56583241-c347-40de-ac69-ad7634985ce8.sh
2020-02-11T02:55:31.8166075Z 
2020-02-11T02:55:31.8169124Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-11T02:55:31.8175662Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69046/merge to s
2020-02-11T02:55:31.8177291Z Task         : Get sources
2020-02-11T02:55:31.8177325Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-11T02:55:31.8177356Z Version      : 1.0.0
2020-02-11T02:55:31.8177435Z Author       : Microsoft
---
2020-02-11T02:55:36.7334707Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-11T02:55:36.7547897Z ##[command]git config gc.auto 0
2020-02-11T02:55:36.7626904Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-11T02:55:36.7689475Z ##[command]git config --get-all http.proxy
2020-02-11T02:55:36.7845278Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69046/merge:refs/remotes/pull/69046/merge
