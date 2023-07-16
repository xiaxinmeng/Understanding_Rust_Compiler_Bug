plain
2020-02-12T15:51:43.0188717Z ========================== Starting Command Output ===========================
2020-02-12T15:51:43.0190100Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/11c79fed-5083-47ba-b044-3cf0c7b10966.sh
2020-02-12T15:51:43.0190135Z 
2020-02-12T15:51:43.0192578Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-12T15:51:43.0198411Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69079/merge to s
2020-02-12T15:51:43.0200961Z Task         : Get sources
2020-02-12T15:51:43.0200992Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-12T15:51:43.0201071Z Version      : 1.0.0
2020-02-12T15:51:43.0201103Z Author       : Microsoft
---
2020-02-12T15:51:43.9662207Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-12T15:51:43.9750228Z ##[command]git config gc.auto 0
2020-02-12T15:51:43.9824787Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-12T15:51:43.9882297Z ##[command]git config --get-all http.proxy
2020-02-12T15:51:44.0033191Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69079/merge:refs/remotes/pull/69079/merge
---
2020-02-12T15:56:29.8165551Z     | ------- previous doc comment
2020-02-12T15:56:29.8165827Z 317 | #![feature(layout_for_ptr)]
2020-02-12T15:56:29.8166177Z     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ not permitted following an outer attibute
2020-02-12T15:56:29.8166411Z     |
2020-02-12T15:56:29.8166845Z     = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-02-12T15:56:29.8208249Z error: an inner attribute is not permitted following an outer doc comment
2020-02-12T15:56:29.8208568Z    --> src/libcore/mem/mod.rs:429:1
2020-02-12T15:56:29.8208795Z     |
2020-02-12T15:56:29.8209045Z 428 | /// 