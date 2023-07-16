plain
2020-02-02T12:29:53.6992893Z ========================== Starting Command Output ===========================
2020-02-02T12:29:53.6994619Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/63e80b83-af7f-403f-9f5c-9845f9d19677.sh
2020-02-02T12:29:53.6994658Z 
2020-02-02T12:29:53.6999317Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-02T12:29:53.7006047Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68767/merge to s
2020-02-02T12:29:53.7007654Z Task         : Get sources
2020-02-02T12:29:53.7007690Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-02T12:29:53.7007848Z Version      : 1.0.0
2020-02-02T12:29:53.7007883Z Author       : Microsoft
---
2020-02-02T12:29:54.7002577Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-02T12:29:54.7016133Z ##[command]git config gc.auto 0
2020-02-02T12:29:54.7018875Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-02T12:29:54.7021230Z ##[command]git config --get-all http.proxy
2020-02-02T12:29:54.7031788Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68767/merge:refs/remotes/pull/68767/merge
