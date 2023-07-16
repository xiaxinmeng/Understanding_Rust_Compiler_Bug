plain
2019-10-18T20:50:54.2775472Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-18T20:50:54.2940242Z ##[command]git config gc.auto 0
2019-10-18T20:50:54.3008018Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-18T20:50:54.3058822Z ##[command]git config --get-all http.proxy
2019-10-18T20:50:54.3170605Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65565/merge:refs/remotes/pull/65565/merge
2019-10-18T20:50:55.0223774Z fatal: couldn't find remote ref refs/pull/65565/merge
2019-10-18T20:50:55.1206977Z ##[warning]Git fetch failed with exit code 128, back off 4.902 seconds before retry.
2019-10-18T20:50:59.9502764Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65565/merge:refs/remotes/pull/65565/merge
2019-10-18T20:51:00.4984740Z fatal: couldn't find remote ref refs/pull/65565/merge
2019-10-18T20:51:00.5443474Z ##[warning]Git fetch failed with exit code 128, back off 8.778 seconds before retry.
2019-10-18T20:51:09.2780672Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65565/merge:refs/remotes/pull/65565/merge
2019-10-18T20:51:09.7834575Z fatal: couldn't find remote ref refs/pull/65565/merge
2019-10-18T20:51:09.8323895Z ##[error]Git fetch failed with exit code: 128
2019-10-18T20:51:09.8561491Z ##[section]Starting: Checkout
2019-10-18T20:51:09.8563175Z ==============================================================================
2019-10-18T20:51:09.8563223Z Task         : Get sources
2019-10-18T20:51:09.8563256Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
