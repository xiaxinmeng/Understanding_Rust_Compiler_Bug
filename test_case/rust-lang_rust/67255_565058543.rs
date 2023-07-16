plain
2019-12-12T15:26:47.6346571Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-12T15:26:47.6565442Z ##[command]git config gc.auto 0
2019-12-12T15:26:47.6577191Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-12T15:26:47.6631335Z ##[command]git config --get-all http.proxy
2019-12-12T15:26:47.6757337Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67255/merge:refs/remotes/pull/67255/merge
---
2019-12-12T15:33:23.1877964Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2019-12-12T15:33:23.1886448Z error[E0583]: file not found for module `i686_unknown_dragonfly`
2019-12-12T15:33:23.1887057Z    --> src/librustc_target/spec/mod.rs:401:32
2019-12-12T15:33:23.1887272Z     |
2019-12-12T15:33:23.1887571Z 401 |     ("i686-unknown-dragonfly", i686_unknown_dragonfly),
2019-12-12T15:33:23.1888089Z     |
2019-12-12T15:33:23.1888089Z     |
2019-12-12T15:33:23.1888429Z     = help: name the file either i686_unknown_dragonfly.rs or i686_unknown_dragonfly/mod.rs inside the directory "src/librustc_target/spec"
2019-12-12T15:33:23.1888731Z error: aborting due to previous error
2019-12-12T15:33:23.1888764Z 
2019-12-12T15:33:23.1889017Z For more information about this error, try `rustc --explain E0583`.
2019-12-12T15:33:23.1889264Z error: could not compile `rustc_target`.
---
2019-12-12T15:33:23.4814283Z   local time: Thu Dec 12 15:33:23 UTC 2019
2019-12-12T15:33:23.6325753Z   network time: Thu, 12 Dec 2019 15:33:23 GMT
2019-12-12T15:33:23.6332000Z == end clock drift check ==
2019-12-12T15:33:25.6883563Z 
2019-12-12T15:33:25.6940217Z ##[error]Bash exited with code '1'.
2019-12-12T15:33:25.6968577Z ##[section]Starting: Checkout
2019-12-12T15:33:25.6970096Z ==============================================================================
2019-12-12T15:33:25.6970146Z Task         : Get sources
2019-12-12T15:33:25.6970208Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
