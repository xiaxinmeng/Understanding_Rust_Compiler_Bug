plain
2019-10-19T11:13:25.0954786Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-19T11:13:25.1139115Z ##[command]git config gc.auto 0
2019-10-19T11:13:25.1215024Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-19T11:13:25.1270600Z ##[command]git config --get-all http.proxy
2019-10-19T11:13:25.1400341Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63793/merge:refs/remotes/pull/63793/merge
---
2019-10-19T11:48:25.8008377Z    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-10-19T11:49:06.1830565Z error: unnecessary `unsafe` block
2019-10-19T11:49:06.1832047Z   --> src/librustc/ty/codec.rs:79:24
2019-10-19T11:49:06.1833536Z    |
2019-10-19T11:49:06.1834182Z 79 |     let discriminant = unsafe { intrinsics::discriminant_value(variant) };
2019-10-19T11:49:06.1836228Z    |
2019-10-19T11:49:06.1836766Z    = note: `-D unused-unsafe` implied by `-D warnings`
2019-10-19T11:49:06.1841007Z 
2019-10-19T11:49:06.1952488Z error: aborting due to previous error
---
2019-10-19T11:49:14.0381325Z   local time: Sat Oct 19 11:49:14 UTC 2019
2019-10-19T11:49:14.3338027Z   network time: Sat, 19 Oct 2019 11:49:14 GMT
2019-10-19T11:49:14.3344786Z == end clock drift check ==
2019-10-19T11:49:15.1328017Z 
2019-10-19T11:49:15.1508772Z ##[error]Bash exited with code '1'.
2019-10-19T11:49:15.1555112Z ##[section]Starting: Checkout
2019-10-19T11:49:15.1557065Z ==============================================================================
2019-10-19T11:49:15.1557115Z Task         : Get sources
2019-10-19T11:49:15.1557174Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
