plain
2020-03-14T12:43:03.9649207Z ========================== Starting Command Output ===========================
2020-03-14T12:43:03.9654031Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4e8085ac-bc39-44ea-9d27-21d7e0204a51.sh
2020-03-14T12:43:03.9654481Z 
2020-03-14T12:43:03.9658430Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-14T12:43:03.9677807Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69985/merge to s
2020-03-14T12:43:03.9681612Z Task         : Get sources
2020-03-14T12:43:03.9681918Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-14T12:43:03.9682196Z Version      : 1.0.0
2020-03-14T12:43:03.9682384Z Author       : Microsoft
---
2020-03-14T12:43:05.4955789Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-14T12:43:05.4979476Z ##[command]git config gc.auto 0
2020-03-14T12:43:05.4984488Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-14T12:43:05.4989801Z ##[command]git config --get-all http.proxy
2020-03-14T12:43:05.4999642Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69985/merge:refs/remotes/pull/69985/merge
