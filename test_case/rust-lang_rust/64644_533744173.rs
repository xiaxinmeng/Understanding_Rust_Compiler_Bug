plain
2019-09-20T23:32:49.9061273Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-20T23:32:49.9337416Z ##[command]git config gc.auto 0
2019-09-20T23:32:49.9402113Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-20T23:32:49.9447386Z ##[command]git config --get-all http.proxy
2019-09-20T23:32:49.9588581Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64644/merge:refs/remotes/pull/64644/merge
2019-09-20T23:32:50.7518225Z fatal: couldn't find remote ref refs/pull/64644/merge
2019-09-20T23:32:50.8527804Z ##[warning]Git fetch failed with exit code 128, back off 1.21 seconds before retry.
2019-09-20T23:32:51.9918028Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64644/merge:refs/remotes/pull/64644/merge
2019-09-20T23:32:52.5977624Z fatal: couldn't find remote ref refs/pull/64644/merge
2019-09-20T23:32:52.6495423Z ##[warning]Git fetch failed with exit code 128, back off 6.495 seconds before retry.
2019-09-20T23:32:59.0968301Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64644/merge:refs/remotes/pull/64644/merge
2019-09-20T23:32:59.6913466Z fatal: couldn't find remote ref refs/pull/64644/merge
2019-09-20T23:32:59.7377314Z ##[error]Git fetch failed with exit code: 128
2019-09-20T23:32:59.7666888Z ##[section]Starting: Checkout
2019-09-20T23:32:59.7668410Z ==============================================================================
2019-09-20T23:32:59.7668464Z Task         : Get sources
2019-09-20T23:32:59.7668502Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
