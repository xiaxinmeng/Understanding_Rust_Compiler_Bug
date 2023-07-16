plain
2019-10-06T00:31:36.9881678Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-06T00:31:37.0118604Z ##[command]git config gc.auto 0
2019-10-06T00:31:37.0202136Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-06T00:31:37.0273569Z ##[command]git config --get-all http.proxy
2019-10-06T00:31:37.0422394Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65082/merge:refs/remotes/pull/65082/merge
---
2019-10-06T00:39:29.9609852Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-10-06T00:39:31.4397033Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-10-06T00:39:32.6915192Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-10-06T00:39:45.1769093Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-10-06T00:39:48.7275906Z error[E0425]: cannot find value `is_reify_shim` in this scope
2019-10-06T00:39:48.7277443Z    --> src/librustc/ty/instance.rs:311:12
2019-10-06T00:39:48.7278419Z 311 |         if is_reify_shim {
2019-10-06T00:39:48.7279135Z     |            ^^^^^^^^^^^^^ not found in this scope
2019-10-06T00:39:48.7279379Z 
2019-10-06T00:40:09.9384321Z error: aborting due to previous error
---
2019-10-06T00:40:10.1237428Z == clock drift check ==
2019-10-06T00:40:10.1255978Z   local time: Sun Oct  6 00:40:10 UTC 2019
2019-10-06T00:40:10.2664912Z   network time: Sun, 06 Oct 2019 00:40:10 GMT
2019-10-06T00:40:10.2669930Z == end clock drift check ==
2019-10-06T00:40:11.0935435Z ##[error]Bash exited with code '1'.
2019-10-06T00:40:11.0967494Z ##[section]Starting: Checkout
2019-10-06T00:40:11.0969078Z ==============================================================================
2019-10-06T00:40:11.0969517Z Task         : Get sources
2019-10-06T00:40:11.0969573Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
