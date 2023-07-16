plain
2019-09-20T23:32:45.8924666Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-20T23:32:46.6070088Z ##[command]git config gc.auto 0
2019-09-20T23:32:46.6073553Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-20T23:32:46.6075509Z ##[command]git config --get-all http.proxy
2019-09-20T23:32:46.6078388Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64643/merge:refs/remotes/pull/64643/merge
2019-09-20T23:32:46.7609235Z fatal: couldn't find remote ref refs/pull/64643/merge
2019-09-20T23:32:46.8664000Z ##[warning]Git fetch failed with exit code 128, back off 3.85 seconds before retry.
2019-09-20T23:32:50.6447360Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64643/merge:refs/remotes/pull/64643/merge
2019-09-20T23:32:51.2445181Z fatal: couldn't find remote ref refs/pull/64643/merge
2019-09-20T23:32:51.3051856Z ##[warning]Git fetch failed with exit code 128, back off 8.654 seconds before retry.
2019-09-20T23:32:59.9025983Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64643/merge:refs/remotes/pull/64643/merge
2019-09-20T23:33:00.5180188Z fatal: couldn't find remote ref refs/pull/64643/merge
2019-09-20T23:33:00.5759399Z ##[error]Git fetch failed with exit code: 128
2019-09-20T23:33:00.6113499Z ##[section]Starting: Checkout
2019-09-20T23:33:00.6115781Z ==============================================================================
2019-09-20T23:33:00.6115856Z Task         : Get sources
2019-09-20T23:33:00.6115904Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
