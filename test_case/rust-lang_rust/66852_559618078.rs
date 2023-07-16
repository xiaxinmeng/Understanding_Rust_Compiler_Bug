plain
2019-11-28T21:30:02.9235768Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-28T21:30:02.9506568Z ##[command]git config gc.auto 0
2019-11-28T21:30:02.9581149Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-28T21:30:02.9639979Z ##[command]git config --get-all http.proxy
2019-11-28T21:30:02.9812294Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66852/merge:refs/remotes/pull/66852/merge
---
2019-11-28T21:39:02.1071424Z configure: build.locked-deps    := True
2019-11-28T21:39:02.1071474Z configure: llvm.ccache          := sccache
2019-11-28T21:39:02.1071733Z configure: build.cargo-native-static := True
2019-11-28T21:39:02.1071973Z configure: dist.missing-tools   := True
2019-11-28T21:39:02.1072280Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-11-28T21:39:02.1072402Z configure: writing `config.toml` in current directory
2019-11-28T21:39:02.1072446Z configure: 
2019-11-28T21:39:02.1072695Z configure: run `python /checkout/x.py --help`
2019-11-28T21:39:02.1072760Z configure: 
---
2019-11-28T21:40:51.6589988Z   local time: Thu Nov 28 21:40:51 UTC 2019
2019-11-28T21:40:51.8072491Z   network time: Thu, 28 Nov 2019 21:40:51 GMT
2019-11-28T21:40:51.8076351Z == end clock drift check ==
2019-11-28T21:41:00.1575275Z 
2019-11-28T21:41:00.1683211Z ##[error]Bash exited with code '1'.
2019-11-28T21:41:00.1713985Z ##[section]Starting: Checkout
2019-11-28T21:41:00.1715479Z ==============================================================================
2019-11-28T21:41:00.1715526Z Task         : Get sources
2019-11-28T21:41:00.1715569Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
