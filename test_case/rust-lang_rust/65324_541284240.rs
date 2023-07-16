plain
2019-10-12T04:22:46.0853686Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-12T04:22:46.0864154Z ##[command]git config gc.auto 0
2019-10-12T04:22:46.0865909Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-12T04:22:46.0868952Z ##[command]git config --get-all http.proxy
2019-10-12T04:22:46.0919222Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65324/merge:refs/remotes/pull/65324/merge
---
2019-10-12T04:29:19.4627227Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-10-12T04:29:27.1221309Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-10-12T04:29:28.4260381Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-10-12T04:29:29.5546598Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-10-12T04:29:37.4840879Z     Checking syntax_parse v0.0.0 (/checkout/src/libsyntax_parse)
2019-10-12T04:29:39.7066615Z error: the feature `mem_take` has been stable since 1.40.0 and no longer requires an attribute to enable
2019-10-12T04:29:39.7067082Z  --> src/libsyntax_parse/lib.rs:4:12
2019-10-12T04:29:39.7067589Z 4 | #![feature(mem_take)]
2019-10-12T04:29:39.7067870Z   |            ^^^^^^^^
2019-10-12T04:29:39.7068084Z   |
2019-10-12T04:29:39.7068362Z   = note: `-D stable-features` implied by `-D warnings`
---
2019-10-12T04:30:21.3618305Z == clock drift check ==
2019-10-12T04:30:21.3636228Z   local time: Sat Oct 12 04:30:21 UTC 2019
2019-10-12T04:30:21.6347635Z   network time: Sat, 12 Oct 2019 04:30:21 GMT
2019-10-12T04:30:21.6350972Z == end clock drift check ==
2019-10-12T04:30:22.2002282Z ##[error]Bash exited with code '1'.
2019-10-12T04:30:22.2049169Z ##[section]Starting: Checkout
2019-10-12T04:30:22.2050819Z ==============================================================================
2019-10-12T04:30:22.2050881Z Task         : Get sources
2019-10-12T04:30:22.2050917Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
