plain
2020-02-20T20:12:31.4419574Z ========================== Starting Command Output ===========================
2020-02-20T20:12:31.4424156Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b823da70-5739-4655-aa3c-d3c8040518c7.sh
2020-02-20T20:12:31.4424471Z 
2020-02-20T20:12:31.4434435Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-20T20:12:31.4479988Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69326/merge to s
2020-02-20T20:12:31.4485450Z Task         : Get sources
2020-02-20T20:12:31.4485761Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-20T20:12:31.4486065Z Version      : 1.0.0
2020-02-20T20:12:31.4486292Z Author       : Microsoft
---
2020-02-20T20:12:32.4537629Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-20T20:12:32.4545400Z ##[command]git config gc.auto 0
2020-02-20T20:12:32.4552936Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-20T20:12:32.4559422Z ##[command]git config --get-all http.proxy
2020-02-20T20:12:32.4571125Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69326/merge:refs/remotes/pull/69326/merge
