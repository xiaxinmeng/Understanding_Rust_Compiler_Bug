plain
2019-09-25T21:15:59.9574931Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-25T21:15:59.9763476Z ##[command]git config gc.auto 0
2019-09-25T21:15:59.9825361Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-25T21:15:59.9899145Z ##[command]git config --get-all http.proxy
2019-09-25T21:16:00.0038989Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64782/merge:refs/remotes/pull/64782/merge
---
2019-09-25T21:23:16.1754291Z    Compiling serde_json v1.0.40
2019-09-25T21:23:18.0825857Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-25T21:23:29.7136508Z     Finished release [optimized] target(s) in 1m 32s
2019-09-25T21:23:29.7232808Z tidy check
2019-09-25T21:23:30.6267858Z tidy error: /checkout/src/libsyntax/ext/mbe/quoted.rs:108: line longer than 100 chars
2019-09-25T21:23:30.6327901Z tidy error: /checkout/src/libsyntax/parse/parser.rs:909: TODO is deprecated; use FIXME
2019-09-25T21:23:30.6380387Z tidy error: /checkout/src/libsyntax/tokenstream.rs:362: line longer than 100 chars
2019-09-25T21:23:31.8434623Z some tidy checks failed
2019-09-25T21:23:31.8438896Z 
2019-09-25T21:23:31.8438896Z 
2019-09-25T21:23:31.8440216Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-25T21:23:31.8441240Z 
2019-09-25T21:23:31.8441333Z 
2019-09-25T21:23:31.8448732Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-25T21:23:31.8448934Z Build completed unsuccessfully in 0:01:35
2019-09-25T21:23:31.8448934Z Build completed unsuccessfully in 0:01:35
2019-09-25T21:23:31.8506334Z == clock drift check ==
2019-09-25T21:23:31.8527118Z   local time: Wed Sep 25 21:23:31 UTC 2019
2019-09-25T21:23:32.1310304Z   network time: Wed, 25 Sep 2019 21:23:32 GMT
2019-09-25T21:23:32.1312829Z == end clock drift check ==
2019-09-25T21:23:33.5161474Z ##[error]Bash exited with code '1'.
2019-09-25T21:23:33.5204735Z ##[section]Starting: Checkout
2019-09-25T21:23:33.5206752Z ==============================================================================
2019-09-25T21:23:33.5206810Z Task         : Get sources
2019-09-25T21:23:33.5206878Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
