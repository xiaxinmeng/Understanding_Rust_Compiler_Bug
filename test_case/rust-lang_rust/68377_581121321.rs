plain
2020-02-02T10:04:45.8885680Z ========================== Starting Command Output ===========================
2020-02-02T10:04:45.8887218Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/662a5518-14cc-438a-844c-17759fe81fcf.sh
2020-02-02T10:04:45.8887256Z 
2020-02-02T10:04:45.8889514Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-02T10:04:45.8894844Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68377/merge to s
2020-02-02T10:04:45.8896223Z Task         : Get sources
2020-02-02T10:04:45.8896250Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-02T10:04:45.8896276Z Version      : 1.0.0
2020-02-02T10:04:45.8896302Z Author       : Microsoft
---
2020-02-02T10:04:46.6806499Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-02T10:04:46.6881384Z ##[command]git config gc.auto 0
2020-02-02T10:04:46.6947462Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-02T10:04:46.7018746Z ##[command]git config --get-all http.proxy
2020-02-02T10:04:46.7153227Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68377/merge:refs/remotes/pull/68377/merge
