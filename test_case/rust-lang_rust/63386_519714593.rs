plain
2019-08-08T20:38:46.1035386Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-08T20:38:46.1236031Z ##[command]git config gc.auto 0
2019-08-08T20:38:46.1324131Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-08T20:38:46.1384750Z ##[command]git config --get-all http.proxy
2019-08-08T20:38:46.1535353Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63386/merge:refs/remotes/pull/63386/merge
2019-08-08T20:38:47.0079577Z fatal: couldn't find remote ref refs/pull/63386/merge
2019-08-08T20:38:47.1166153Z ##[warning]Git fetch failed with exit code 128, back off 3.057 seconds before retry.
2019-08-08T20:38:50.1037177Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63386/merge:refs/remotes/pull/63386/merge
2019-08-08T20:38:50.7223671Z fatal: couldn't find remote ref refs/pull/63386/merge
2019-08-08T20:38:50.7909062Z ##[warning]Git fetch failed with exit code 128, back off 2.263 seconds before retry.
2019-08-08T20:38:52.9874813Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63386/merge:refs/remotes/pull/63386/merge
2019-08-08T20:38:53.6238351Z fatal: couldn't find remote ref refs/pull/63386/merge
2019-08-08T20:38:53.6757538Z ##[error]Git fetch failed with exit code: 128
2019-08-08T20:38:53.7085152Z ##[section]Starting: Checkout
2019-08-08T20:38:53.7086794Z ==============================================================================
2019-08-08T20:38:53.7086842Z Task         : Get sources
2019-08-08T20:38:53.7086882Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
