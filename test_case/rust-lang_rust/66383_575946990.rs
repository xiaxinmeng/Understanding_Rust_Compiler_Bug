plain
2020-01-18T23:13:29.9371066Z ========================== Starting Command Output ===========================
2020-01-18T23:13:29.9372710Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0572d766-0fd2-45c7-9fb9-e282641d0eba.sh
2020-01-18T23:13:29.9372801Z 
2020-01-18T23:13:29.9376367Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-18T23:13:29.9382781Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/66383/merge to s
2020-01-18T23:13:29.9386101Z Task         : Get sources
2020-01-18T23:13:29.9386191Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T23:13:29.9386225Z Version      : 1.0.0
2020-01-18T23:13:29.9386259Z Author       : Microsoft
---
2020-01-18T23:13:30.9125386Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-18T23:13:30.9226098Z ##[command]git config gc.auto 0
2020-01-18T23:13:30.9937672Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-18T23:13:31.0004835Z ##[command]git config --get-all http.proxy
2020-01-18T23:13:31.0181579Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66383/merge:refs/remotes/pull/66383/merge
