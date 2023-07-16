plain
2019-11-25T20:45:53.0855302Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-25T20:45:53.1052846Z ##[command]git config gc.auto 0
2019-11-25T20:45:53.1113352Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-25T20:45:53.1166719Z ##[command]git config --get-all http.proxy
2019-11-25T20:45:53.9038391Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66755/merge:refs/remotes/pull/66755/merge
---
2019-11-25T21:14:37.3809786Z    Compiling rustc-demangle v0.1.16
2019-11-25T21:14:37.5150795Z error[E0658]: `if` is not allowed in a `const fn`
2019-11-25T21:14:37.5152859Z   --> src/liballoc/raw_vec.rs:58:19
2019-11-25T21:14:37.5154721Z    |
2019-11-25T21:14:37.5156345Z 58 |         let cap = if mem::size_of::<T>() == 0 { !0 } else { 0 };
2019-11-25T21:14:37.5159796Z    |
2019-11-25T21:14:37.5159796Z    |
2019-11-25T21:14:37.5161551Z    = note: for more information, see ***/issues/49146
2019-11-25T21:14:37.5163309Z    = help: add `#![feature(const_if_match)]` to the crate attributes to enable
2019-11-25T21:14:37.5181835Z error[E0658]: `if` is not allowed in a `const fn`
2019-11-25T21:14:37.5182159Z    --> src/liballoc/raw_vec.rs:149:19
2019-11-25T21:14:37.5182394Z     |
2019-11-25T21:14:37.5182394Z     |
2019-11-25T21:14:37.5182689Z 149 |         let cap = if mem::size_of::<T>() == 0 { !0 } else { 0 };
2019-11-25T21:14:37.5183194Z     |
2019-11-25T21:14:37.5183194Z     |
2019-11-25T21:14:37.5183552Z     = note: for more information, see ***/issues/49146
2019-11-25T21:14:37.5184526Z     = help: add `#![feature(const_if_match)]` to the crate attributes to enable
2019-11-25T21:14:38.7350065Z error: aborting due to 2 previous errors
2019-11-25T21:14:38.7350878Z 
2019-11-25T21:14:38.7355581Z For more information about this error, try `rustc --explain E0658`.
2019-11-25T21:14:38.7545689Z error: could not compile `alloc`.
---
2019-11-25T21:14:39.0212563Z   local time: Mon Nov 25 21:14:39 UTC 2019
2019-11-25T21:14:39.2994430Z   network time: Mon, 25 Nov 2019 21:14:39 GMT
2019-11-25T21:14:39.2994634Z == end clock drift check ==
2019-11-25T21:14:42.1186679Z 
2019-11-25T21:14:42.1294886Z ##[error]Bash exited with code '1'.
2019-11-25T21:14:42.1329579Z ##[section]Starting: Checkout
2019-11-25T21:14:42.1331264Z ==============================================================================
2019-11-25T21:14:42.1331329Z Task         : Get sources
2019-11-25T21:14:42.1331370Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
