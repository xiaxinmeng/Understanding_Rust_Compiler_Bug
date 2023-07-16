plain
2019-10-30T09:36:16.1852998Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-30T09:36:16.2063756Z ##[command]git config gc.auto 0
2019-10-30T09:36:16.2116531Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-30T09:36:16.2164471Z ##[command]git config --get-all http.proxy
2019-10-30T09:36:16.2289835Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65958/merge:refs/remotes/pull/65958/merge
2019-10-30T09:36:17.1226078Z fatal: couldn't find remote ref refs/pull/65958/merge
2019-10-30T09:36:17.1922927Z ##[warning]Git fetch failed with exit code 128, back off 3.055 seconds before retry.
2019-10-30T09:36:20.1170684Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65958/merge:refs/remotes/pull/65958/merge
2019-10-30T09:36:20.7516793Z fatal: couldn't find remote ref refs/pull/65958/merge
2019-10-30T09:36:20.8046404Z ##[warning]Git fetch failed with exit code 128, back off 5.416 seconds before retry.
2019-10-30T09:36:26.1693104Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65958/merge:refs/remotes/pull/65958/merge
2019-10-30T09:36:26.7579699Z fatal: couldn't find remote ref refs/pull/65958/merge
2019-10-30T09:36:26.8087422Z ##[error]Git fetch failed with exit code: 128
2019-10-30T09:36:26.8349452Z ##[section]Starting: Checkout
2019-10-30T09:36:26.8350863Z ==============================================================================
2019-10-30T09:36:26.8350904Z Task         : Get sources
2019-10-30T09:36:26.8350936Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
