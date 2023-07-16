plain
2019-11-08T21:49:15.9833787Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-08T21:49:16.0060228Z ##[command]git config gc.auto 0
2019-11-08T21:49:16.0133713Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-08T21:49:16.0222883Z ##[command]git config --get-all http.proxy
2019-11-08T21:49:16.0398919Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66215/merge:refs/remotes/pull/66215/merge
---
2019-11-08T21:55:50.6199468Z    Compiling serde_json v1.0.40
2019-11-08T21:55:52.5706216Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-08T21:56:04.6905266Z     Finished release [optimized] target(s) in 1m 33s
2019-11-08T21:56:04.6995950Z tidy check
2019-11-08T21:56:05.5277279Z tidy error: /checkout/src/liballoc/string.rs:2003: line longer than 100 chars
2019-11-08T21:56:07.5825647Z Found 485 error codes
2019-11-08T21:56:07.5828917Z Found 0 error codes with no tests
2019-11-08T21:56:07.5829053Z Done!
2019-11-08T21:56:07.5829103Z some tidy checks failed
2019-11-08T21:56:07.5829103Z some tidy checks failed
2019-11-08T21:56:07.5832145Z 
2019-11-08T21:56:07.5832224Z 
2019-11-08T21:56:07.5833277Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-08T21:56:07.5833413Z 
2019-11-08T21:56:07.5833439Z 
2019-11-08T21:56:07.5841063Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-08T21:56:07.5841133Z Build completed unsuccessfully in 0:01:37
2019-11-08T21:56:07.5841133Z Build completed unsuccessfully in 0:01:37
2019-11-08T21:56:07.5898017Z == clock drift check ==
2019-11-08T21:56:07.5907909Z   local time: Fri Nov  8 21:56:07 UTC 2019
2019-11-08T21:56:07.6770189Z   network time: Fri, 08 Nov 2019 21:56:07 GMT
2019-11-08T21:56:07.6770757Z == end clock drift check ==
2019-11-08T21:56:09.0512944Z 
2019-11-08T21:56:09.0630920Z ##[error]Bash exited with code '1'.
2019-11-08T21:56:09.0659655Z ##[section]Starting: Checkout
2019-11-08T21:56:09.0684554Z ==============================================================================
2019-11-08T21:56:09.0684617Z Task         : Get sources
2019-11-08T21:56:09.0684679Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
