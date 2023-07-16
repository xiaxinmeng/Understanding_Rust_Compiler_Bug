plain
2020-01-25T09:17:19.9015714Z ========================== Starting Command Output ===========================
2020-01-25T09:17:19.9286712Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/97bd428e-4e2d-4ffa-ab15-9308dca4244c.sh
2020-01-25T09:17:19.9286859Z 
2020-01-25T09:17:19.9289439Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-25T09:17:19.9294149Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67330/merge to s
2020-01-25T09:17:19.9295500Z Task         : Get sources
2020-01-25T09:17:19.9295529Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-25T09:17:19.9295594Z Version      : 1.0.0
2020-01-25T09:17:19.9295623Z Author       : Microsoft
---
2020-01-25T09:17:20.6947463Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-25T09:17:20.7052747Z ##[command]git config gc.auto 0
2020-01-25T09:17:20.7092283Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-25T09:17:20.7285153Z ##[command]git config --get-all http.proxy
2020-01-25T09:17:20.7292810Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67330/merge:refs/remotes/pull/67330/merge
