plain
2020-02-15T00:07:22.8388523Z ========================== Starting Command Output ===========================
2020-02-15T00:07:22.8390327Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8d11c8f9-c2f1-471b-80e6-aa561c60e4b8.sh
2020-02-15T00:07:22.8390369Z 
2020-02-15T00:07:22.8393453Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-15T00:07:22.8400919Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68191/merge to s
2020-02-15T00:07:22.8402921Z Task         : Get sources
2020-02-15T00:07:22.8402959Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-15T00:07:22.8402996Z Version      : 1.0.0
2020-02-15T00:07:22.8403032Z Author       : Microsoft
---
2020-02-15T00:07:23.9057697Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-15T00:07:23.9066634Z ##[command]git config gc.auto 0
2020-02-15T00:07:23.9068907Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-15T00:07:23.9071105Z ##[command]git config --get-all http.proxy
2020-02-15T00:07:23.9077618Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68191/merge:refs/remotes/pull/68191/merge
