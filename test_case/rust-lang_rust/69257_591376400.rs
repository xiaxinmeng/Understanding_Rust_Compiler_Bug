plain
2020-02-26T11:01:28.1940167Z ========================== Starting Command Output ===========================
2020-02-26T11:01:28.1944906Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1a1265f2-e122-4cd8-b0df-a1c23813c226.sh
2020-02-26T11:01:28.1945365Z 
2020-02-26T11:01:28.1949401Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-26T11:01:28.1968541Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69257/merge to s
2020-02-26T11:01:28.1971962Z Task         : Get sources
2020-02-26T11:01:28.1972257Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-26T11:01:28.1972539Z Version      : 1.0.0
2020-02-26T11:01:28.1972750Z Author       : Microsoft
---
2020-02-26T11:01:30.8349807Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-26T11:01:30.8863879Z ##[command]git config gc.auto 0
2020-02-26T11:01:30.8868782Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-26T11:01:30.8874999Z ##[command]git config --get-all http.proxy
2020-02-26T11:01:30.8898714Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69257/merge:refs/remotes/pull/69257/merge
