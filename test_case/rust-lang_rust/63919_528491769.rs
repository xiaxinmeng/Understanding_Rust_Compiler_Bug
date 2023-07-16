plain
2019-09-05T17:19:26.2547684Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-05T17:19:26.2770659Z ##[command]git config gc.auto 0
2019-09-05T17:19:26.2854802Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-05T17:19:26.2926569Z ##[command]git config --get-all http.proxy
2019-09-05T17:19:26.3085896Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63919/merge:refs/remotes/pull/63919/merge
---
2019-09-05T17:28:00.2930156Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-09-05T17:28:01.8425849Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-09-05T17:28:03.1590715Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-09-05T17:28:17.0039422Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-09-05T17:28:17.3034532Z error[E0425]: cannot find value `import_path` in this scope
2019-09-05T17:28:17.3034999Z   --> src/libsyntax_ext/standard_library_imports.rs:77:35
2019-09-05T17:28:17.3035239Z    |
2019-09-05T17:28:17.3035537Z 77 |             prefix: cx.path(span, import_path),
2019-09-05T17:28:17.3036011Z 
2019-09-05T17:28:18.1597135Z error: aborting due to previous error
2019-09-05T17:28:18.1598038Z 
2019-09-05T17:28:18.1609085Z For more information about this error, try `rustc --explain E0425`.
---
2019-09-05T17:29:12.7676633Z == clock drift check ==
2019-09-05T17:29:12.7691719Z   local time: Thu Sep  5 17:29:12 UTC 2019
2019-09-05T17:29:12.9256255Z   network time: Thu, 05 Sep 2019 17:29:12 GMT
2019-09-05T17:29:12.9256596Z == end clock drift check ==
2019-09-05T17:29:14.0196044Z ##[error]Bash exited with code '1'.
2019-09-05T17:29:14.0234727Z ##[section]Starting: Checkout
2019-09-05T17:29:14.0236505Z ==============================================================================
2019-09-05T17:29:14.0236563Z Task         : Get sources
2019-09-05T17:29:14.0236631Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
