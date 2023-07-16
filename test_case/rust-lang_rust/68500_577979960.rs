plain
2020-01-24T01:52:32.1959341Z ========================== Starting Command Output ===========================
2020-01-24T01:52:32.1963673Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9bebaeac-65bc-447e-91ac-78df9d88da1f.sh
2020-01-24T01:52:32.1963962Z 
2020-01-24T01:52:32.1969914Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-24T01:52:32.1977350Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68500/merge to s
2020-01-24T01:52:32.1979423Z Task         : Get sources
2020-01-24T01:52:32.1979461Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-24T01:52:32.1979497Z Version      : 1.0.0
2020-01-24T01:52:32.1979534Z Author       : Microsoft
---
2020-01-24T01:52:33.2504418Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-24T01:52:33.2518578Z ##[command]git config gc.auto 0
2020-01-24T01:52:33.2520927Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-24T01:52:33.2522916Z ##[command]git config --get-all http.proxy
2020-01-24T01:52:33.2529817Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68500/merge:refs/remotes/pull/68500/merge
