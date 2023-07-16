plain
2019-07-24T14:34:25.4364706Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-24T14:34:25.4583900Z ##[command]git config gc.auto 0
2019-07-24T14:34:25.4642755Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-24T14:34:25.4687123Z ##[command]git config --get-all http.proxy
2019-07-24T14:34:25.4865747Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62938/merge:refs/remotes/pull/62938/merge
2019-07-24T14:34:26.2728534Z fatal: couldn't find remote ref refs/pull/62938/merge
2019-07-24T14:34:26.3791656Z ##[warning]Git fetch failed with exit code 128, back off 1.761 seconds before retry.
2019-07-24T14:34:28.0706138Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62938/merge:refs/remotes/pull/62938/merge
2019-07-24T14:34:28.7040412Z fatal: couldn't find remote ref refs/pull/62938/merge
2019-07-24T14:34:28.7655738Z ##[warning]Git fetch failed with exit code 128, back off 9.15 seconds before retry.
2019-07-24T14:34:37.8546518Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62938/merge:refs/remotes/pull/62938/merge
2019-07-24T14:34:38.5386418Z fatal: couldn't find remote ref refs/pull/62938/merge
2019-07-24T14:34:38.5900036Z ##[error]Git fetch failed with exit code: 128
2019-07-24T14:34:38.6206713Z ##[section]Starting: Checkout
2019-07-24T14:34:38.6208279Z ==============================================================================
2019-07-24T14:34:38.6208333Z Task         : Get sources
2019-07-24T14:34:38.6208386Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
