plain
2019-09-04T02:11:19.2949771Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-04T02:11:19.3126006Z ##[command]git config gc.auto 0
2019-09-04T02:11:19.3212673Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-04T02:11:19.3262594Z ##[command]git config --get-all http.proxy
2019-09-04T02:11:19.9005435Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64138/merge:refs/remotes/pull/64138/merge
2019-09-04T02:11:21.0976533Z fatal: remote error: upload-pack: not our ref 34619a742575a8470af3447b05ce428d969a73d5
2019-09-04T02:11:21.1085179Z fatal: the remote end hung up unexpectedly
2019-09-04T02:11:21.2369767Z ##[warning]Git fetch failed with exit code 128, back off 1.246 seconds before retry.
2019-09-04T02:11:22.3918734Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64138/merge:refs/remotes/pull/64138/merge
2019-09-04T02:11:22.9997614Z fatal: couldn't find remote ref refs/pull/64138/merge
2019-09-04T02:11:23.0861966Z ##[warning]Git fetch failed with exit code 128, back off 9.425 seconds before retry.
2019-09-04T02:11:32.4256885Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64138/merge:refs/remotes/pull/64138/merge
2019-09-04T02:11:33.0419244Z fatal: couldn't find remote ref refs/pull/64138/merge
2019-09-04T02:11:33.1037513Z ##[error]Git fetch failed with exit code: 128
2019-09-04T02:11:33.1359127Z ##[section]Starting: Checkout
2019-09-04T02:11:33.1361352Z ==============================================================================
2019-09-04T02:11:33.1361410Z Task         : Get sources
2019-09-04T02:11:33.1361471Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
