plain
2020-01-18T00:09:27.8278748Z ========================== Starting Command Output ===========================
2020-01-18T00:09:27.8280396Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/5f8a4a81-d1d0-4182-bc87-bca63d09ab62.sh
2020-01-18T00:09:27.8280437Z 
2020-01-18T00:09:27.8283905Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-18T00:09:27.8290640Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68204/merge to s
2020-01-18T00:09:27.8292559Z Task         : Get sources
2020-01-18T00:09:27.8292585Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-18T00:09:27.8292634Z Version      : 1.0.0
2020-01-18T00:09:27.8292659Z Author       : Microsoft
---
2020-01-18T00:09:28.8234499Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-18T00:09:28.8245654Z ##[command]git config gc.auto 0
2020-01-18T00:09:28.8247925Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-18T00:09:28.8251402Z ##[command]git config --get-all http.proxy
2020-01-18T00:09:28.8257106Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68204/merge:refs/remotes/pull/68204/merge
