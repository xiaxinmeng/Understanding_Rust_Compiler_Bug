plain
2019-08-15T17:14:04.6507007Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-15T17:14:04.6741547Z ##[command]git config gc.auto 0
2019-08-15T17:14:04.7128500Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-15T17:14:04.7178385Z ##[command]git config --get-all http.proxy
2019-08-15T17:14:04.7316691Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63606/merge:refs/remotes/pull/63606/merge
2019-08-15T17:14:05.4930758Z fatal: couldn't find remote ref refs/pull/63606/merge
2019-08-15T17:14:05.5883475Z ##[warning]Git fetch failed with exit code 128, back off 1.358 seconds before retry.
2019-08-15T17:14:06.8817785Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63606/merge:refs/remotes/pull/63606/merge
2019-08-15T17:14:07.5208889Z fatal: couldn't find remote ref refs/pull/63606/merge
2019-08-15T17:14:07.5720823Z ##[warning]Git fetch failed with exit code 128, back off 4.668 seconds before retry.
2019-08-15T17:14:12.1879379Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63606/merge:refs/remotes/pull/63606/merge
2019-08-15T17:14:12.7688708Z fatal: couldn't find remote ref refs/pull/63606/merge
2019-08-15T17:14:12.8141838Z ##[error]Git fetch failed with exit code: 128
2019-08-15T17:14:12.8419800Z ##[section]Starting: Checkout
2019-08-15T17:14:12.8421160Z ==============================================================================
2019-08-15T17:14:12.8421206Z Task         : Get sources
2019-08-15T17:14:12.8421244Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
