plain
2019-11-12T09:05:13.9835160Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-12T09:05:14.0024337Z ##[command]git config gc.auto 0
2019-11-12T09:05:14.0101243Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-12T09:05:14.0157660Z ##[command]git config --get-all http.proxy
2019-11-12T09:05:14.0299805Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66325/merge:refs/remotes/pull/66325/merge
---
2019-11-12T09:34:59.7277294Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-11-12T09:35:16.5537372Z error: unused label
2019-11-12T09:35:16.5537710Z     --> src/librustc/ty/layout.rs:2560:25
2019-11-12T09:35:16.5538096Z      |
2019-11-12T09:35:16.5538361Z 2560 |                         'iter_fields: for i in 0..fat_pointer_layout.fields.count() {
2019-11-12T09:35:16.5538793Z      |
2019-11-12T09:35:16.5539011Z      = note: `-D unused-labels` implied by `-D warnings`
2019-11-12T09:35:16.5545918Z 
2019-11-12T09:35:34.3924686Z error: aborting due to previous error
---
2019-11-12T09:35:42.4005192Z   local time: Tue Nov 12 09:35:42 UTC 2019
2019-11-12T09:35:42.5491154Z   network time: Tue, 12 Nov 2019 09:35:42 GMT
2019-11-12T09:35:42.5495299Z == end clock drift check ==
2019-11-12T09:35:43.4731838Z 
2019-11-12T09:35:43.4813736Z ##[error]Bash exited with code '1'.
2019-11-12T09:35:43.4840991Z ##[section]Starting: Checkout
2019-11-12T09:35:43.4842740Z ==============================================================================
2019-11-12T09:35:43.4842795Z Task         : Get sources
2019-11-12T09:35:43.4842860Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
