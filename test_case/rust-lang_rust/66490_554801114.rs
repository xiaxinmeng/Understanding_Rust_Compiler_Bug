plain
2019-11-17T23:20:40.9234271Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-17T23:20:40.9428763Z ##[command]git config gc.auto 0
2019-11-17T23:20:40.9524278Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-17T23:20:40.9570018Z ##[command]git config --get-all http.proxy
2019-11-17T23:20:40.9739920Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66490/merge:refs/remotes/pull/66490/merge
---
2019-11-17T23:31:55.1392887Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2019-11-17T23:31:55.9835104Z error[E0277]: cannot add `&syntax_pos::symbol::SymbolStr` to `std::string::String`
2019-11-17T23:31:55.9835494Z     --> src/librustc_parse/parser/expr.rs:1113:52
2019-11-17T23:31:55.9835782Z      |
2019-11-17T23:31:55.9836100Z 1113 |                         let s = String::from("0.") + &symbol.as_str();
2019-11-17T23:31:55.9836557Z      |                                                    ^ no implementation for `std::string::String + &syntax_pos::symbol::SymbolStr`
2019-11-17T23:31:55.9836792Z      |
2019-11-17T23:31:55.9837141Z      = help: the trait `std::ops::Add<&syntax_pos::symbol::SymbolStr>` is not implemented for `std::string::String`
2019-11-17T23:31:55.9958292Z error[E0277]: the size for values of type `str` cannot be known at compilation time
2019-11-17T23:31:55.9958684Z     --> src/librustc_parse/parser/expr.rs:1114:80
2019-11-17T23:31:55.9958991Z      |
2019-11-17T23:31:55.9958991Z      |
2019-11-17T23:31:55.9959359Z 1114 |                         let kind = TokenKind::lit(token::Float, Symbol::intern(&s), suffix);
2019-11-17T23:31:55.9959840Z      |                                                                                ^^ doesn't have a size known at compile-time
2019-11-17T23:31:55.9962458Z      = help: the trait `std::marker::Sized` is not implemented for `str`
2019-11-17T23:31:55.9962879Z      = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-11-17T23:31:55.9963214Z      = note: all local variables must have a statically known size
2019-11-17T23:31:55.9963560Z      = help: unsized locals are gated as an unstable feature
---
2019-11-17T23:37:51.0961096Z   local time: Sun Nov 17 23:37:51 UTC 2019
2019-11-17T23:37:51.3787794Z   network time: Sun, 17 Nov 2019 23:37:51 GMT
2019-11-17T23:37:51.3788003Z == end clock drift check ==
2019-11-17T23:37:54.2486192Z 
2019-11-17T23:37:54.2600657Z ##[error]Bash exited with code '1'.
2019-11-17T23:37:54.2631097Z ##[section]Starting: Checkout
2019-11-17T23:37:54.2632931Z ==============================================================================
2019-11-17T23:37:54.2632996Z Task         : Get sources
2019-11-17T23:37:54.2633069Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
