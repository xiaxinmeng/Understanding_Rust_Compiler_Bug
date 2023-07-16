plain
2020-02-05T01:05:00.0349215Z ========================== Starting Command Output ===========================
2020-02-05T01:05:00.0351919Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ec8f368d-9be0-4d3b-95de-9980af4cb39f.sh
2020-02-05T01:05:00.0352070Z 
2020-02-05T01:05:00.0354917Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-05T01:05:00.0360830Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68788/merge to s
2020-02-05T01:05:00.0362453Z Task         : Get sources
2020-02-05T01:05:00.0362530Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-05T01:05:00.0362568Z Version      : 1.0.0
2020-02-05T01:05:00.0362605Z Author       : Microsoft
---
2020-02-05T01:05:00.9913936Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-05T01:05:00.9996659Z ##[command]git config gc.auto 0
2020-02-05T01:05:01.0083997Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-05T01:05:01.0131520Z ##[command]git config --get-all http.proxy
2020-02-05T01:05:01.0281382Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68788/merge:refs/remotes/pull/68788/merge
