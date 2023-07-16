plain
2019-09-27T06:08:14.4315029Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-27T06:08:14.4532957Z ##[command]git config gc.auto 0
2019-09-27T06:08:14.4619167Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-27T06:08:14.4679281Z ##[command]git config --get-all http.proxy
2019-09-27T06:08:14.4826324Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64831/merge:refs/remotes/pull/64831/merge
2019-09-27T06:08:15.2725034Z fatal: couldn't find remote ref refs/pull/64831/merge
2019-09-27T06:08:15.3984228Z ##[warning]Git fetch failed with exit code 128, back off 8.481 seconds before retry.
2019-09-27T06:08:23.7890099Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64831/merge:refs/remotes/pull/64831/merge
2019-09-27T06:08:24.3896457Z fatal: couldn't find remote ref refs/pull/64831/merge
2019-09-27T06:08:24.4398297Z ##[warning]Git fetch failed with exit code 128, back off 7.054 seconds before retry.
2019-09-27T06:08:31.4461462Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64831/merge:refs/remotes/pull/64831/merge
2019-09-27T06:08:32.0419097Z fatal: couldn't find remote ref refs/pull/64831/merge
2019-09-27T06:08:32.1018502Z ##[error]Git fetch failed with exit code: 128
2019-09-27T06:08:32.1319493Z ##[section]Starting: Checkout
2019-09-27T06:08:32.1321092Z ==============================================================================
2019-09-27T06:08:32.1321155Z Task         : Get sources
2019-09-27T06:08:32.1321193Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
