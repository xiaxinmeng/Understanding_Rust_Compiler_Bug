plain
2019-11-22T14:46:48.5976259Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-22T14:46:48.6159874Z ##[command]git config gc.auto 0
2019-11-22T14:46:48.6234781Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-22T14:46:48.6283533Z ##[command]git config --get-all http.proxy
2019-11-22T14:46:48.6424305Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66498/merge:refs/remotes/pull/66498/merge
---
2019-11-22T14:53:12.3031345Z    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-11-22T14:53:16.5581549Z error[E0658]: dereferencing raw pointers in constant functions is unstable
2019-11-22T14:53:16.5583300Z     --> src/libstd/ffi/c_str.rs:1070:9
2019-11-22T14:53:16.5585326Z      |
2019-11-22T14:53:16.5585962Z 1070 |         &*(bytes as *const [u8] as *const CStr)
2019-11-22T14:53:16.5587074Z      |
2019-11-22T14:53:16.5587074Z      |
2019-11-22T14:53:16.5587673Z      = note: for more information, see ***/issues/51911
2019-11-22T14:53:16.5588841Z 
2019-11-22T14:53:17.7301316Z error: aborting due to previous error
2019-11-22T14:53:17.7302260Z 
2019-11-22T14:53:17.7303571Z For more information about this error, try `rustc --explain E0658`.
---
2019-11-22T14:53:17.7943795Z   local time: Fri Nov 22 14:53:17 UTC 2019
2019-11-22T14:53:18.0714658Z   network time: Fri, 22 Nov 2019 14:53:18 GMT
2019-11-22T14:53:18.0714913Z == end clock drift check ==
2019-11-22T14:53:20.9435547Z 
2019-11-22T14:53:20.9544676Z ##[error]Bash exited with code '1'.
2019-11-22T14:53:20.9571052Z ##[section]Starting: Checkout
2019-11-22T14:53:20.9573214Z ==============================================================================
2019-11-22T14:53:20.9573296Z Task         : Get sources
2019-11-22T14:53:20.9573345Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
