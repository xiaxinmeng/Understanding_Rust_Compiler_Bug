plain
2019-10-01T21:58:17.3477123Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-01T21:58:17.3668977Z ##[command]git config gc.auto 0
2019-10-01T21:58:17.3738111Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-01T21:58:17.3801087Z ##[command]git config --get-all http.proxy
2019-10-01T21:58:17.3938063Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64971/merge:refs/remotes/pull/64971/merge
2019-10-01T21:58:18.1554993Z fatal: couldn't find remote ref refs/pull/64971/merge
2019-10-01T21:58:18.2797771Z ##[warning]Git fetch failed with exit code 128, back off 9.997 seconds before retry.
2019-10-01T21:58:28.1911448Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64971/merge:refs/remotes/pull/64971/merge
2019-10-01T21:58:28.7876234Z fatal: couldn't find remote ref refs/pull/64971/merge
2019-10-01T21:58:28.8395694Z ##[warning]Git fetch failed with exit code 128, back off 1.705 seconds before retry.
2019-10-01T21:58:30.4971691Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64971/merge:refs/remotes/pull/64971/merge
2019-10-01T21:58:31.1045922Z fatal: couldn't find remote ref refs/pull/64971/merge
2019-10-01T21:58:31.1555504Z ##[error]Git fetch failed with exit code: 128
2019-10-01T21:58:31.1868969Z ##[section]Starting: Checkout
2019-10-01T21:58:31.1870581Z ==============================================================================
2019-10-01T21:58:31.1870636Z Task         : Get sources
2019-10-01T21:58:31.1870688Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
