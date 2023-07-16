plain
2019-12-12T18:55:21.6838740Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-12T18:55:22.6761514Z ##[command]git config gc.auto 0
2019-12-12T18:55:22.6766255Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-12T18:55:22.6769435Z ##[command]git config --get-all http.proxy
2019-12-12T18:55:22.6772100Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67241/merge:refs/remotes/pull/67241/merge
---
2019-12-12T19:01:13.7457246Z    Compiling serde_json v1.0.40
2019-12-12T19:01:15.6177252Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-12T19:01:26.7016792Z     Finished release [optimized] target(s) in 1m 29s
2019-12-12T19:01:26.7132531Z tidy check
2019-12-12T19:01:27.6541168Z tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1566: TODO is deprecated; use FIXME
2019-12-12T19:01:27.6712107Z tidy error: /checkout/src/librustc_mir/borrow_check/region_infer/mod.rs:485: TODO is deprecated; use FIXME
2019-12-12T19:01:29.5139704Z some tidy checks failed
2019-12-12T19:01:29.5140577Z Found 485 error codes
2019-12-12T19:01:29.5141020Z Found 0 error codes with no tests
2019-12-12T19:01:29.5141323Z Done!
2019-12-12T19:01:29.5141323Z Done!
2019-12-12T19:01:29.5144904Z 
2019-12-12T19:01:29.5145245Z 
2019-12-12T19:01:29.5146466Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-12T19:01:29.5147087Z 
2019-12-12T19:01:29.5147312Z 
2019-12-12T19:01:29.5154327Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-12T19:01:29.5154407Z Build completed unsuccessfully in 0:01:33
2019-12-12T19:01:29.5154407Z Build completed unsuccessfully in 0:01:33
2019-12-12T19:01:29.5213732Z == clock drift check ==
2019-12-12T19:01:29.5223739Z   local time: Thu Dec 12 19:01:29 UTC 2019
2019-12-12T19:01:30.3167237Z   network time: Thu, 12 Dec 2019 19:01:29 GMT
2019-12-12T19:01:30.3167862Z == end clock drift check ==
2019-12-12T19:01:30.9172804Z 
2019-12-12T19:01:30.9259162Z ##[error]Bash exited with code '1'.
2019-12-12T19:01:30.9290340Z ##[section]Starting: Checkout
2019-12-12T19:01:30.9292716Z ==============================================================================
2019-12-12T19:01:30.9292794Z Task         : Get sources
2019-12-12T19:01:30.9292847Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
