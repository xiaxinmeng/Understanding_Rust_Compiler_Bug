plain
2019-08-24T19:05:29.3716837Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-24T19:05:29.3926604Z ##[command]git config gc.auto 0
2019-08-24T19:05:29.4003198Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-24T19:05:29.4065047Z ##[command]git config --get-all http.proxy
2019-08-24T19:05:30.1765647Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63863/merge:refs/remotes/pull/63863/merge
2019-08-24T19:05:30.2326372Z fatal: couldn't find remote ref refs/pull/63863/merge
2019-08-24T19:05:30.3470331Z ##[warning]Git fetch failed with exit code 128, back off 4.315 seconds before retry.
2019-08-24T19:05:34.5909605Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63863/merge:refs/remotes/pull/63863/merge
2019-08-24T19:05:35.2045439Z fatal: couldn't find remote ref refs/pull/63863/merge
2019-08-24T19:05:35.2779647Z ##[warning]Git fetch failed with exit code 128, back off 6.301 seconds before retry.
2019-08-24T19:05:41.5117102Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63863/merge:refs/remotes/pull/63863/merge
2019-08-24T19:05:42.1276230Z fatal: couldn't find remote ref refs/pull/63863/merge
2019-08-24T19:05:42.1858125Z ##[error]Git fetch failed with exit code: 128
2019-08-24T19:05:42.2141188Z ##[section]Starting: Checkout
2019-08-24T19:05:42.2142704Z ==============================================================================
2019-08-24T19:05:42.2142765Z Task         : Get sources
2019-08-24T19:05:42.2142806Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
