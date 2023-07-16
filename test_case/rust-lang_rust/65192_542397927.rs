plain
2019-10-15T20:38:33.7795619Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-15T20:38:33.8004478Z ##[command]git config gc.auto 0
2019-10-15T20:38:33.8084326Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-15T20:38:33.8153198Z ##[command]git config --get-all http.proxy
2019-10-15T20:38:34.3484411Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65192/merge:refs/remotes/pull/65192/merge
---
2019-10-15T20:47:21.9181794Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-10-15T20:47:40.2920840Z error[E0277]: can't compare `&std::string::String` with `syntax::symbol::LocalInternedString`
2019-10-15T20:47:40.2921421Z     --> src/librustc/traits/error_reporting.rs:1063:37
2019-10-15T20:47:40.2921744Z      |
2019-10-15T20:47:40.2922058Z 1063 |                         &param_name == p.name.ident().as_str()
2019-10-15T20:47:40.2922854Z      |                                     ^^ no implementation for `&std::string::String == syntax::symbol::LocalInternedString`
2019-10-15T20:47:40.2923539Z      = help: the trait `std::cmp::PartialEq<syntax::symbol::LocalInternedString>` is not implemented for `&std::string::String`
2019-10-15T20:47:40.2923613Z 
2019-10-15T20:47:48.5058751Z error: aborting due to previous error
2019-10-15T20:47:48.5058998Z 
---
2019-10-15T20:47:48.6911785Z == clock drift check ==
2019-10-15T20:47:48.6929415Z   local time: Tue Oct 15 20:47:48 UTC 2019
2019-10-15T20:47:48.8424059Z   network time: Tue, 15 Oct 2019 20:47:48 GMT
2019-10-15T20:47:48.8429648Z == end clock drift check ==
2019-10-15T20:47:49.6183061Z ##[error]Bash exited with code '1'.
2019-10-15T20:47:49.6237584Z ##[section]Starting: Checkout
2019-10-15T20:47:49.6239475Z ==============================================================================
2019-10-15T20:47:49.6239534Z Task         : Get sources
2019-10-15T20:47:49.6239583Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
