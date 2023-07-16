plain
2020-01-21T06:38:37.1923205Z ========================== Starting Command Output ===========================
2020-01-21T06:38:37.1925491Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/62254e3e-c034-4cdf-845f-18ee933ed685.sh
2020-01-21T06:38:37.1925688Z 
2020-01-21T06:38:37.1930006Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-21T06:38:37.1937177Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68406/merge to s
2020-01-21T06:38:37.1939321Z Task         : Get sources
2020-01-21T06:38:37.1939356Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-21T06:38:37.1939400Z Version      : 1.0.0
2020-01-21T06:38:37.1939435Z Author       : Microsoft
---
2020-01-21T06:38:38.0271251Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-21T06:38:38.0367478Z ##[command]git config gc.auto 0
2020-01-21T06:38:38.0452946Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-21T06:38:38.0509888Z ##[command]git config --get-all http.proxy
2020-01-21T06:38:38.0645438Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68406/merge:refs/remotes/pull/68406/merge
