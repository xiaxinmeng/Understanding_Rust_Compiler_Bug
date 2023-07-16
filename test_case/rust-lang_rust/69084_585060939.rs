plain
2020-02-12T06:45:21.6918066Z ========================== Starting Command Output ===========================
2020-02-12T06:45:21.7189109Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/5183fbc9-d6b9-4868-b3ea-de2f5e170ca1.sh
2020-02-12T06:45:21.7189311Z 
2020-02-12T06:45:21.7192499Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-12T06:45:21.7198998Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69084/merge to s
2020-02-12T06:45:21.7201454Z Task         : Get sources
2020-02-12T06:45:21.7201491Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-12T06:45:21.7201575Z Version      : 1.0.0
2020-02-12T06:45:21.7201612Z Author       : Microsoft
---
2020-02-12T06:45:22.5581913Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-12T06:45:22.5709064Z ##[command]git config gc.auto 0
2020-02-12T06:45:22.5772649Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-12T06:45:22.5835238Z ##[command]git config --get-all http.proxy
2020-02-12T06:45:22.5975697Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69084/merge:refs/remotes/pull/69084/merge
