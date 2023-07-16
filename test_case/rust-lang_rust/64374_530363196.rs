plain
2019-09-11T10:45:58.4130118Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-11T10:45:59.1803827Z ##[command]git config gc.auto 0
2019-09-11T10:45:59.1807473Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-11T10:45:59.1809463Z ##[command]git config --get-all http.proxy
2019-09-11T10:45:59.1813653Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64374/merge:refs/remotes/pull/64374/merge
---
2019-09-11T10:57:50.5276168Z configure: build.locked-deps    := True
2019-09-11T10:57:50.5276383Z configure: llvm.ccache          := sccache
2019-09-11T10:57:50.5276860Z configure: build.cargo-native-static := True
2019-09-11T10:57:50.5277275Z configure: dist.missing-tools   := True
2019-09-11T10:57:50.5277762Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-09-11T10:57:50.5279622Z configure: writing `config.toml` in current directory
2019-09-11T10:57:50.5279955Z configure: 
2019-09-11T10:57:50.5280630Z configure: run `python /checkout/x.py --help`
2019-09-11T10:57:50.5280946Z configure: 
---
2019-09-11T11:03:17.5879091Z == clock drift check ==
2019-09-11T11:03:17.5901063Z   local time: Wed Sep 11 11:03:17 UTC 2019
2019-09-11T11:03:17.7394505Z   network time: Wed, 11 Sep 2019 11:03:17 GMT
2019-09-11T11:03:17.7398405Z == end clock drift check ==
2019-09-11T11:03:18.8417134Z ##[error]Bash exited with code '1'.
2019-09-11T11:03:18.8500637Z ##[section]Starting: Checkout
2019-09-11T11:03:18.8503320Z ==============================================================================
2019-09-11T11:03:18.8503401Z Task         : Get sources
2019-09-11T11:03:18.8503448Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
