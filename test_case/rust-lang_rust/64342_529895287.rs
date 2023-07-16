plain
2019-09-10T10:35:50.8838504Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-10T10:35:50.9012045Z ##[command]git config gc.auto 0
2019-09-10T10:35:50.9089739Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-10T10:35:50.9146298Z ##[command]git config --get-all http.proxy
2019-09-10T10:35:50.9285705Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64342/merge:refs/remotes/pull/64342/merge
---
2019-09-10T10:45:30.9090967Z configure: build.locked-deps    := True
2019-09-10T10:45:30.9091328Z configure: llvm.ccache          := sccache
2019-09-10T10:45:30.9091770Z configure: build.cargo-native-static := True
2019-09-10T10:45:30.9092043Z configure: dist.missing-tools   := True
2019-09-10T10:45:30.9092265Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-09-10T10:45:30.9092346Z configure: writing `config.toml` in current directory
2019-09-10T10:45:30.9092397Z configure: 
2019-09-10T10:45:30.9175612Z configure: run `python /checkout/x.py --help`
2019-09-10T10:45:30.9175709Z configure: 
---
2019-09-10T10:53:43.5318604Z    Compiling serde v1.0.99
2019-09-10T10:53:51.7801482Z    Compiling serde_json v1.0.40
2019-09-10T10:53:53.4685617Z    Compiling rustfix v0.4.6
2019-09-10T10:53:56.8956702Z    Compiling compiletest v0.0.0 (/checkout/src/tools/compiletest)
2019-09-10T10:53:57.0340599Z error: expected `{`, found `,`
2019-09-10T10:53:57.0346510Z      |
2019-09-10T10:53:57.0346510Z      |
2019-09-10T10:53:57.0348234Z 2495 |                             if pluralise!(v.len()),
2019-09-10T10:53:57.0350846Z      |                             --                    ^ expected `{`
2019-09-10T10:53:57.0354350Z      |                             this `if` statement has a condition, but no block
2019-09-10T10:53:57.0355679Z 
2019-09-10T10:53:57.0535534Z error[E0433]: failed to resolve: use of undeclared type or module `syntax`
2019-09-10T10:53:57.0537450Z   --> src/tools/compiletest/src/runtest.rs:39:5
---
2019-09-10T10:53:57.9084967Z == clock drift check ==
2019-09-10T10:53:57.9104010Z   local time: Tue Sep 10 10:53:57 UTC 2019
2019-09-10T10:53:57.9932267Z   network time: Tue, 10 Sep 2019 10:53:57 GMT
2019-09-10T10:53:57.9936191Z == end clock drift check ==
2019-09-10T10:53:59.3037176Z ##[error]Bash exited with code '1'.
2019-09-10T10:53:59.3072895Z ##[section]Starting: Checkout
2019-09-10T10:53:59.3074833Z ==============================================================================
2019-09-10T10:53:59.3074899Z Task         : Get sources
2019-09-10T10:53:59.3074941Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
