plain
2019-11-18T17:58:12.7349725Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-18T17:58:12.7634613Z ##[command]git config gc.auto 0
2019-11-18T17:58:12.7669908Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-18T17:58:12.7750878Z ##[command]git config --get-all http.proxy
2019-11-18T17:58:12.7956517Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66239/merge:refs/remotes/pull/66239/merge
---
2019-11-18T18:07:44.0861279Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-11-18T18:07:54.1764574Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-11-18T18:07:55.8408181Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-11-18T18:08:16.7974389Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-11-18T18:08:32.0861290Z error: expected one of `.`, `?`, `{`, or an operator, found `;`
2019-11-18T18:08:32.0861926Z      |
2019-11-18T18:08:32.0861926Z      |
2019-11-18T18:08:32.0862217Z 1286 |         let hir_id = match hir.as_local_hir_id(def_id)?;
2019-11-18T18:08:32.0862638Z      |                      -----                             ^ expected one of `.`, `?`, `{`, or an operator here
2019-11-18T18:08:32.0863352Z      |                      while parsing this match expression
2019-11-18T18:08:32.0863616Z      |                      help: try removing this `match`
2019-11-18T18:08:32.0863742Z 
2019-11-18T18:08:55.6809783Z error: aborting due to previous error
---
2019-11-18T18:08:58.2837941Z   local time: Mon Nov 18 18:08:58 UTC 2019
2019-11-18T18:08:58.5678289Z   network time: Mon, 18 Nov 2019 18:08:58 GMT
2019-11-18T18:08:58.5682653Z == end clock drift check ==
2019-11-18T18:08:59.6844864Z 
2019-11-18T18:08:59.6949833Z ##[error]Bash exited with code '1'.
2019-11-18T18:08:59.6979766Z ##[section]Starting: Checkout
2019-11-18T18:08:59.6981509Z ==============================================================================
2019-11-18T18:08:59.6981562Z Task         : Get sources
2019-11-18T18:08:59.6981607Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
