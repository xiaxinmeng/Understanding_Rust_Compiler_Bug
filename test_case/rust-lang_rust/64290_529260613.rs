plain
2019-09-09T00:07:39.4641081Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-09T00:07:39.4848004Z ##[command]git config gc.auto 0
2019-09-09T00:07:39.4944713Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-09T00:07:39.5015175Z ##[command]git config --get-all http.proxy
2019-09-09T00:07:39.5161906Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64290/merge:refs/remotes/pull/64290/merge
---
2019-09-09T00:16:28.7190126Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-09-09T00:16:32.3088758Z error[E0425]: cannot find function `main_err` in this scope
2019-09-09T00:16:32.3089534Z    --> src/librustc/middle/entry.rs:151:9
2019-09-09T00:16:32.3089842Z     |
2019-09-09T00:16:32.3090140Z 151 |         main_err(tcx, visitor);
2019-09-09T00:16:32.3090653Z 
2019-09-09T00:16:56.3235202Z error: aborting due to previous error
2019-09-09T00:16:56.3235585Z 
2019-09-09T00:16:56.3235987Z For more information about this error, try `rustc --explain E0425`.
---
2019-09-09T00:16:56.5269007Z == clock drift check ==
2019-09-09T00:16:56.5286462Z   local time: Mon Sep  9 00:16:56 UTC 2019
2019-09-09T00:16:56.6790405Z   network time: Mon, 09 Sep 2019 00:16:56 GMT
2019-09-09T00:16:56.6793373Z == end clock drift check ==
2019-09-09T00:16:57.3502658Z ##[error]Bash exited with code '1'.
2019-09-09T00:16:57.3539283Z ##[section]Starting: Checkout
2019-09-09T00:16:57.3541129Z ==============================================================================
2019-09-09T00:16:57.3541188Z Task         : Get sources
2019-09-09T00:16:57.3541257Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
