plain
2020-02-18T01:00:56.0092437Z ========================== Starting Command Output ===========================
2020-02-18T01:00:56.0095351Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1eec2dd7-002a-47aa-98cd-b0076075d7c4.sh
2020-02-18T01:00:56.0095402Z 
2020-02-18T01:00:56.0098762Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-18T01:00:56.0104670Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69252/merge to s
2020-02-18T01:00:56.0106540Z Task         : Get sources
2020-02-18T01:00:56.0106567Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-18T01:00:56.0106594Z Version      : 1.0.0
2020-02-18T01:00:56.0106630Z Author       : Microsoft
---
2020-02-18T01:00:56.8542922Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-18T01:00:56.8631688Z ##[command]git config gc.auto 0
2020-02-18T01:00:56.8718144Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-18T01:00:56.8769728Z ##[command]git config --get-all http.proxy
2020-02-18T01:00:56.8904634Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69252/merge:refs/remotes/pull/69252/merge
