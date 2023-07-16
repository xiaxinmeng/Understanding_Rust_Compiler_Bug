plain
2020-01-06T08:55:46.6878981Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-06T08:55:46.7081119Z ##[command]git config gc.auto 0
2020-01-06T08:55:46.7156443Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-06T08:55:46.7209891Z ##[command]git config --get-all http.proxy
2020-01-06T08:55:46.7364146Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67921/merge:refs/remotes/pull/67921/merge
2020-01-06T08:55:47.8753303Z fatal: couldn't find remote ref refs/pull/67921/merge
2020-01-06T08:55:47.9476173Z ##[warning]Git fetch failed with exit code 128, back off 6.433 seconds before retry.
2020-01-06T08:55:53.8976063Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67921/merge:refs/remotes/pull/67921/merge
2020-01-06T08:55:54.4376032Z fatal: couldn't find remote ref refs/pull/67921/merge
2020-01-06T08:55:54.5006339Z ##[warning]Git fetch failed with exit code 128, back off 9.464 seconds before retry.
2020-01-06T08:56:03.9015696Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67921/merge:refs/remotes/pull/67921/merge
2020-01-06T08:56:04.4623364Z fatal: couldn't find remote ref refs/pull/67921/merge
2020-01-06T08:56:04.5167779Z ##[error]Git fetch failed with exit code: 128
2020-01-06T08:56:04.5359521Z ##[section]Starting: Checkout
2020-01-06T08:56:04.5361914Z ==============================================================================
2020-01-06T08:56:04.5362002Z Task         : Get sources
2020-01-06T08:56:04.5362049Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
