plain
2020-02-18T21:52:00.6655933Z ========================== Starting Command Output ===========================
2020-02-18T21:52:00.6658634Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/bd66bf0e-3652-4e85-a594-b21eed28e8db.sh
2020-02-18T21:52:00.6658718Z 
2020-02-18T21:52:00.6663109Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-18T21:52:00.6670753Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-02-18T21:52:00.6673176Z Task         : Get sources
2020-02-18T21:52:00.6673208Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-18T21:52:00.6673239Z Version      : 1.0.0
2020-02-18T21:52:00.6673315Z Author       : Microsoft
---
2020-02-18T21:52:02.5428204Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-18T21:52:02.5442045Z ##[command]git config gc.auto 0
2020-02-18T21:52:02.5446607Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-18T21:52:02.5449111Z ##[command]git config --get-all http.proxy
2020-02-18T21:52:02.5454489Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69274/merge:refs/remotes/pull/69274/merge
