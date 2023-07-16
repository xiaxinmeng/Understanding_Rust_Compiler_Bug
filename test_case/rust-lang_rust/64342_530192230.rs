plain
2019-09-11T01:19:58.3482919Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-11T01:19:58.3707004Z ##[command]git config gc.auto 0
2019-09-11T01:19:58.3786901Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-11T01:19:58.3846351Z ##[command]git config --get-all http.proxy
2019-09-11T01:19:58.3988655Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64342/merge:refs/remotes/pull/64342/merge
---
2019-09-11T01:31:04.4813763Z configure: build.locked-deps    := True
2019-09-11T01:31:04.4813854Z configure: llvm.ccache          := sccache
2019-09-11T01:31:04.4814110Z configure: build.cargo-native-static := True
2019-09-11T01:31:04.4814387Z configure: dist.missing-tools   := True
2019-09-11T01:31:04.4816109Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-09-11T01:31:04.4816282Z configure: writing `config.toml` in current directory
2019-09-11T01:31:04.4816333Z configure: 
2019-09-11T01:31:04.4816900Z configure: run `python /checkout/x.py --help`
2019-09-11T01:31:04.4816989Z configure: 
---
2019-09-11T01:41:10.6865104Z    |
2019-09-11T01:41:10.6871514Z 39 | use syntax::errors::pluralise;
2019-09-11T01:41:10.6876480Z    |     ^^^^^^ use of undeclared type or module `syntax`
2019-09-11T01:41:10.6880703Z 
2019-09-11T01:41:10.6994838Z error: cannot find macro `pluralise!` in this scope
2019-09-11T01:41:10.7002861Z      |
2019-09-11T01:41:10.7002861Z      |
2019-09-11T01:41:10.7003551Z 2495 |                             pluralise!(v.len()),
2019-09-11T01:41:10.7005464Z 
2019-09-11T01:41:10.7547335Z error: unused import: `syntax::errors::pluralise`
2019-09-11T01:41:10.7548877Z   --> src/tools/compiletest/src/runtest.rs:39:5
2019-09-11T01:41:10.7549707Z    |
---
2019-09-11T01:41:11.7414451Z == clock drift check ==
2019-09-11T01:41:11.7434036Z   local time: Wed Sep 11 01:41:11 UTC 2019
2019-09-11T01:41:11.8298576Z   network time: Wed, 11 Sep 2019 01:41:11 GMT
2019-09-11T01:41:11.8301247Z == end clock drift check ==
2019-09-11T01:41:13.1783705Z ##[error]Bash exited with code '1'.
2019-09-11T01:41:13.1825292Z ##[section]Starting: Checkout
2019-09-11T01:41:13.1827651Z ==============================================================================
2019-09-11T01:41:13.1827732Z Task         : Get sources
2019-09-11T01:41:13.1827780Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
