plain
2019-09-26T01:58:25.2564638Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-26T01:58:25.2775635Z ##[command]git config gc.auto 0
2019-09-26T01:58:25.2859845Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-26T01:58:25.2918568Z ##[command]git config --get-all http.proxy
2019-09-26T01:58:25.3095066Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64795/merge:refs/remotes/pull/64795/merge
2019-09-26T01:58:26.1625484Z fatal: couldn't find remote ref refs/pull/64795/merge
2019-09-26T01:58:26.2980306Z ##[warning]Git fetch failed with exit code 128, back off 1.963 seconds before retry.
2019-09-26T01:58:28.1700971Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64795/merge:refs/remotes/pull/64795/merge
2019-09-26T01:58:28.8053957Z fatal: couldn't find remote ref refs/pull/64795/merge
2019-09-26T01:58:28.8731839Z ##[warning]Git fetch failed with exit code 128, back off 4.262 seconds before retry.
2019-09-26T01:58:33.0732689Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64795/merge:refs/remotes/pull/64795/merge
2019-09-26T01:58:33.6988322Z fatal: couldn't find remote ref refs/pull/64795/merge
2019-09-26T01:58:33.7643364Z ##[error]Git fetch failed with exit code: 128
2019-09-26T01:58:33.8050405Z ##[section]Starting: Checkout
2019-09-26T01:58:33.8053172Z ==============================================================================
2019-09-26T01:58:33.8053243Z Task         : Get sources
2019-09-26T01:58:33.8053288Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
