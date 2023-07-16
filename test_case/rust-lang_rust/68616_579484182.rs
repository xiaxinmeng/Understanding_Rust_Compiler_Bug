plain
2020-01-28T21:55:54.7082886Z ========================== Starting Command Output ===========================
2020-01-28T21:55:54.7084880Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d9b21cd8-feae-4e1f-bada-6990b98c3a4a.sh
2020-01-28T21:55:54.7084906Z 
2020-01-28T21:55:54.7087019Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-28T21:55:54.7091093Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68616/merge to s
2020-01-28T21:55:54.7092221Z Task         : Get sources
2020-01-28T21:55:54.7092247Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-28T21:55:54.7092305Z Version      : 1.0.0
2020-01-28T21:55:54.7092331Z Author       : Microsoft
---
2020-01-28T21:55:55.5260508Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-28T21:55:55.5270378Z ##[command]git config gc.auto 0
2020-01-28T21:55:55.5272927Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-28T21:55:55.5274216Z ##[command]git config --get-all http.proxy
2020-01-28T21:55:55.5446480Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68616/merge:refs/remotes/pull/68616/merge
