plain
2020-01-23T03:03:13.5157595Z ========================== Starting Command Output ===========================
2020-01-23T03:03:13.5215146Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b81baf3f-9bd2-49ad-acbc-7e99ab6bf6ae.sh
2020-01-23T03:03:13.5215426Z 
2020-01-23T03:03:13.5218776Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-23T03:03:13.5226306Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68447/merge to s
2020-01-23T03:03:13.5228463Z Task         : Get sources
2020-01-23T03:03:13.5228560Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-23T03:03:13.5228604Z Version      : 1.0.0
2020-01-23T03:03:13.5228646Z Author       : Microsoft
---
2020-01-23T03:03:14.5454245Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-23T03:03:14.5464966Z ##[command]git config gc.auto 0
2020-01-23T03:03:14.5467540Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-23T03:03:14.5469645Z ##[command]git config --get-all http.proxy
2020-01-23T03:03:14.5476236Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68447/merge:refs/remotes/pull/68447/merge
