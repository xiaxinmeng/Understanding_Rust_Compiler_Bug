plain
2019-11-15T17:01:41.6527566Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-15T17:01:41.6782339Z ##[command]git config gc.auto 0
2019-11-15T17:01:41.6849638Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-15T17:01:41.6909874Z ##[command]git config --get-all http.proxy
2019-11-15T17:01:41.7072829Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66452/merge:refs/remotes/pull/66452/merge
2019-11-15T17:01:42.8293753Z fatal: couldn't find remote ref refs/pull/66452/merge
2019-11-15T17:01:42.9031081Z ##[warning]Git fetch failed with exit code 128, back off 1.012 seconds before retry.
2019-11-15T17:01:43.6396122Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66452/merge:refs/remotes/pull/66452/merge
2019-11-15T17:01:44.3743227Z fatal: couldn't find remote ref refs/pull/66452/merge
2019-11-15T17:01:44.4317531Z ##[warning]Git fetch failed with exit code 128, back off 4.729 seconds before retry.
2019-11-15T17:01:49.1087756Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66452/merge:refs/remotes/pull/66452/merge
2019-11-15T17:01:49.8356066Z fatal: couldn't find remote ref refs/pull/66452/merge
2019-11-15T17:01:49.8880963Z ##[error]Git fetch failed with exit code: 128
2019-11-15T17:01:49.9053369Z ##[section]Starting: Checkout
2019-11-15T17:01:49.9054843Z ==============================================================================
2019-11-15T17:01:49.9054894Z Task         : Get sources
2019-11-15T17:01:49.9054936Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
