plain
2019-08-06T08:28:04.8942950Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-06T08:28:04.9158898Z ##[command]git config gc.auto 0
2019-08-06T08:28:04.9213198Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-06T08:28:04.9271229Z ##[command]git config --get-all http.proxy
2019-08-06T08:28:04.9426255Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63321/merge:refs/remotes/pull/63321/merge
2019-08-06T08:28:06.1158120Z fatal: couldn't find remote ref refs/pull/63321/merge
2019-08-06T08:28:06.2161685Z ##[warning]Git fetch failed with exit code 128, back off 1.005 seconds before retry.
2019-08-06T08:28:07.1540254Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63321/merge:refs/remotes/pull/63321/merge
2019-08-06T08:28:07.7379889Z fatal: couldn't find remote ref refs/pull/63321/merge
2019-08-06T08:28:07.8001641Z ##[warning]Git fetch failed with exit code 128, back off 3.718 seconds before retry.
2019-08-06T08:28:11.4605318Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63321/merge:refs/remotes/pull/63321/merge
2019-08-06T08:28:12.1251925Z fatal: couldn't find remote ref refs/pull/63321/merge
2019-08-06T08:28:12.1780469Z ##[error]Git fetch failed with exit code: 128
2019-08-06T08:28:12.2064301Z ##[section]Starting: Checkout
2019-08-06T08:28:12.2065850Z ==============================================================================
2019-08-06T08:28:12.2065903Z Task         : Get sources
2019-08-06T08:28:12.2065947Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
