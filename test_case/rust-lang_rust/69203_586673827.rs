plain
2020-02-16T06:01:18.4959682Z ========================== Starting Command Output ===========================
2020-02-16T06:01:18.4961770Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/46337de9-14f7-4ba1-b383-1ddf36f6356f.sh
2020-02-16T06:01:18.4961868Z 
2020-02-16T06:01:18.4968844Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-16T06:01:18.4974835Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69203/merge to s
2020-02-16T06:01:18.4976564Z Task         : Get sources
2020-02-16T06:01:18.4976598Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-16T06:01:18.4976631Z Version      : 1.0.0
2020-02-16T06:01:18.4976709Z Author       : Microsoft
---
2020-02-16T06:01:22.8031997Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-16T06:01:22.8068169Z ##[command]git config gc.auto 0
2020-02-16T06:01:22.8077863Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-16T06:01:22.8095536Z ##[command]git config --get-all http.proxy
2020-02-16T06:01:22.8126826Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69203/merge:refs/remotes/pull/69203/merge
