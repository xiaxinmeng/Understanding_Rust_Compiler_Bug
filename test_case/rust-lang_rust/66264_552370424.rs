plain
2019-11-11T09:32:26.9741382Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-11T09:32:26.9929038Z ##[command]git config gc.auto 0
2019-11-11T09:32:27.0013709Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-11T09:32:27.0085048Z ##[command]git config --get-all http.proxy
2019-11-11T09:32:27.8243611Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66264/merge:refs/remotes/pull/66264/merge
---
2019-11-11T09:42:34.6231852Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-11-11T09:42:36.1003200Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-11-11T09:42:54.5553599Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-11-11T09:43:43.8111441Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2019-11-11T09:43:44.9998239Z error[E0599]: no method named `map_or` found for type `std::result::Result<std::string::String, syntax_pos::SpanSnippetError>` in the current scope
2019-11-11T09:43:44.9999013Z      |
2019-11-11T09:43:44.9999013Z      |
2019-11-11T09:43:44.9999374Z 1747 |             .map_or(false, |s| s.ends_with(")") || s.ends_with("]"));
2019-11-11T09:43:44.9999792Z      |              ^^^^^^ help: there is a method with a similar name: `map_err`
2019-11-11T09:43:45.4256455Z error: aborting due to previous error
2019-11-11T09:43:45.4260767Z 
2019-11-11T09:43:45.4271955Z For more information about this error, try `rustc --explain E0599`.
2019-11-11T09:43:45.4485982Z error: could not compile `rustc_parse`.
---
2019-11-11T09:49:29.3928886Z   local time: Mon Nov 11 09:49:29 UTC 2019
2019-11-11T09:49:29.5435012Z   network time: Mon, 11 Nov 2019 09:49:29 GMT
2019-11-11T09:49:29.5438079Z == end clock drift check ==
2019-11-11T09:49:32.8126236Z 
2019-11-11T09:49:32.8229820Z ##[error]Bash exited with code '1'.
2019-11-11T09:49:32.8269021Z ##[section]Starting: Checkout
2019-11-11T09:49:32.8270692Z ==============================================================================
2019-11-11T09:49:32.8270747Z Task         : Get sources
2019-11-11T09:49:32.8270815Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
