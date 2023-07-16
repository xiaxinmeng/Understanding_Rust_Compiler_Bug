plain
2020-01-10T06:49:39.5290994Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-10T06:49:39.5305176Z ##[command]git config gc.auto 0
2020-01-10T06:49:39.5310079Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-10T06:49:39.5312091Z ##[command]git config --get-all http.proxy
2020-01-10T06:49:39.5314135Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68086/merge:refs/remotes/pull/68086/merge
2020-01-10T06:49:40.0392840Z fatal: couldn't find remote ref refs/pull/68086/merge
2020-01-10T06:49:40.1451820Z ##[warning]Git fetch failed with exit code 128, back off 6.217 seconds before retry.
2020-01-10T06:49:46.2866207Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68086/merge:refs/remotes/pull/68086/merge
2020-01-10T06:49:46.8427608Z fatal: couldn't find remote ref refs/pull/68086/merge
2020-01-10T06:49:46.8940551Z ##[warning]Git fetch failed with exit code 128, back off 8.124 seconds before retry.
2020-01-10T06:49:54.9673869Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68086/merge:refs/remotes/pull/68086/merge
2020-01-10T06:49:55.5384321Z fatal: couldn't find remote ref refs/pull/68086/merge
2020-01-10T06:49:55.5904001Z ##[error]Git fetch failed with exit code: 128
2020-01-10T06:49:55.6078104Z ##[section]Starting: Checkout
2020-01-10T06:49:55.6079945Z ==============================================================================
2020-01-10T06:49:55.6079996Z Task         : Get sources
2020-01-10T06:49:55.6080038Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
