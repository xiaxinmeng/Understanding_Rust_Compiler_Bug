plain
2019-10-20T18:13:47.7468583Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-20T18:13:47.7672878Z ##[command]git config gc.auto 0
2019-10-20T18:13:47.7756783Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-20T18:13:47.7819491Z ##[command]git config --get-all http.proxy
2019-10-20T18:13:47.7968228Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65631/merge:refs/remotes/pull/65631/merge
2019-10-20T18:13:48.8101952Z fatal: couldn't find remote ref refs/pull/65631/merge
2019-10-20T18:13:48.8780229Z ##[warning]Git fetch failed with exit code 128, back off 1.235 seconds before retry.
2019-10-20T18:13:49.8622337Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65631/merge:refs/remotes/pull/65631/merge
2019-10-20T18:13:50.4830707Z fatal: couldn't find remote ref refs/pull/65631/merge
2019-10-20T18:13:50.5545562Z ##[warning]Git fetch failed with exit code 128, back off 8.424 seconds before retry.
2019-10-20T18:13:58.9118531Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65631/merge:refs/remotes/pull/65631/merge
2019-10-20T18:13:59.5041379Z fatal: couldn't find remote ref refs/pull/65631/merge
2019-10-20T18:13:59.5517551Z ##[error]Git fetch failed with exit code: 128
2019-10-20T18:13:59.5794567Z ##[section]Starting: Checkout
2019-10-20T18:13:59.5796017Z ==============================================================================
2019-10-20T18:13:59.5796063Z Task         : Get sources
2019-10-20T18:13:59.5796099Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
