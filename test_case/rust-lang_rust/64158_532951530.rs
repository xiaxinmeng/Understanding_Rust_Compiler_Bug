plain
2019-09-19T01:43:36.6296169Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-19T01:43:36.6499443Z ##[command]git config gc.auto 0
2019-09-19T01:43:36.6643861Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-19T01:43:37.5588905Z ##[command]git config --get-all http.proxy
2019-09-19T01:43:37.5595444Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64158/merge:refs/remotes/pull/64158/merge
---
2019-09-19T01:53:58.4638243Z configure: build.locked-deps    := True
2019-09-19T01:53:58.4638603Z configure: llvm.ccache          := sccache
2019-09-19T01:53:58.4638955Z configure: build.cargo-native-static := True
2019-09-19T01:53:58.4639215Z configure: dist.missing-tools   := True
2019-09-19T01:53:58.4639557Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2019-09-19T01:53:58.4639735Z configure: writing `config.toml` in current directory
2019-09-19T01:53:58.4639805Z configure: 
2019-09-19T01:53:58.4640080Z configure: run `python /checkout/x.py --help`
2019-09-19T01:53:58.4640133Z configure: 
---
2019-09-19T01:56:20.6221535Z     Checking test v0.0.0 (/checkout/src/libtest)
2019-09-19T01:56:21.0356011Z error[E0308]: mismatched types
2019-09-19T01:56:21.0356446Z     --> src/libtest/lib.rs:1648:25
2019-09-19T01:56:21.0356708Z      |
2019-09-19T01:56:21.0357174Z 1648 |     status.code().ok_or("received no exit code from child process")
2019-09-19T01:56:21.0357788Z      |                         |
2019-09-19T01:56:21.0358127Z      |                         expected struct `std::string::String`, found reference
2019-09-19T01:56:21.0358127Z      |                         expected struct `std::string::String`, found reference
2019-09-19T01:56:21.0358476Z      |                         help: try using a conversion method: `"received no exit code from child process".to_string()`
2019-09-19T01:56:21.0359301Z      = note: expected type `std::string::String`
2019-09-19T01:56:21.0360010Z                 found type `&'static str`
2019-09-19T01:56:21.0360094Z 
2019-09-19T01:56:21.1666526Z error: aborting due to previous error
---
2019-09-19T01:56:21.1942743Z == clock drift check ==
2019-09-19T01:56:21.1963399Z   local time: Thu Sep 19 01:56:21 UTC 2019
2019-09-19T01:56:21.2818350Z   network time: Thu, 19 Sep 2019 01:56:21 GMT
2019-09-19T01:56:21.2822846Z == end clock drift check ==
2019-09-19T01:56:22.7061886Z ##[error]Bash exited with code '1'.
2019-09-19T01:56:22.7106381Z ##[section]Starting: Checkout
2019-09-19T01:56:22.7109247Z ==============================================================================
2019-09-19T01:56:22.7109319Z Task         : Get sources
2019-09-19T01:56:22.7109373Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
