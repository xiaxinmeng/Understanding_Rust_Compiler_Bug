plain
2020-02-03T12:54:23.3310475Z ========================== Starting Command Output ===========================
2020-02-03T12:54:23.3311947Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/501b690b-6e1d-4229-963b-8a47eae39b0c.sh
2020-02-03T12:54:23.3311982Z 
2020-02-03T12:54:23.3314397Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-03T12:54:23.3319882Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68795/merge to s
2020-02-03T12:54:23.3321277Z Task         : Get sources
2020-02-03T12:54:23.3321306Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-03T12:54:23.3321375Z Version      : 1.0.0
2020-02-03T12:54:23.3321405Z Author       : Microsoft
---
2020-02-03T12:54:24.1034191Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-03T12:54:24.1108287Z ##[command]git config gc.auto 0
2020-02-03T12:54:24.1174247Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-03T12:54:24.1223837Z ##[command]git config --get-all http.proxy
2020-02-03T12:54:24.1357808Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68795/merge:refs/remotes/pull/68795/merge
