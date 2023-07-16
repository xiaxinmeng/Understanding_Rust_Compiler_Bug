plain
2019-11-10T17:30:40.9214791Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-10T17:30:40.9450836Z ##[command]git config gc.auto 0
2019-11-10T17:30:40.9540962Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-10T17:30:40.9593698Z ##[command]git config --get-all http.proxy
2019-11-10T17:30:40.9737054Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66277/merge:refs/remotes/pull/66277/merge
---
2019-11-10T17:36:55.9018581Z    Compiling serde_json v1.0.40
2019-11-10T17:36:57.7182159Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-10T17:37:09.3526053Z     Finished release [optimized] target(s) in 1m 30s
2019-11-10T17:37:09.3604907Z tidy check
2019-11-10T17:37:10.5315481Z tidy error: /checkout/src/libcore/num/mod.rs:5267: undocumented unsafe
2019-11-10T17:37:10.5315594Z tidy error: /checkout/src/libcore/num/mod.rs:5286: line longer than 100 chars
2019-11-10T17:37:10.5315644Z tidy error: /checkout/src/libcore/num/mod.rs:5287: line longer than 100 chars
2019-11-10T17:37:10.5315867Z tidy error: /checkout/src/libcore/num/mod.rs:5288: line longer than 100 chars
2019-11-10T17:37:10.5315912Z tidy error: /checkout/src/libcore/num/mod.rs:5289: line longer than 100 chars
2019-11-10T17:37:10.5315955Z tidy error: /checkout/src/libcore/num/mod.rs:5290: line longer than 100 chars
2019-11-10T17:37:10.5316013Z tidy error: /checkout/src/libcore/num/mod.rs:5291: line longer than 100 chars
2019-11-10T17:37:10.5316057Z tidy error: /checkout/src/libcore/num/mod.rs:5292: line longer than 100 chars
2019-11-10T17:37:10.5316101Z tidy error: /checkout/src/libcore/num/mod.rs:5293: line longer than 100 chars
2019-11-10T17:37:10.5316160Z tidy error: /checkout/src/libcore/num/mod.rs:5294: line longer than 100 chars
2019-11-10T17:37:10.5316477Z tidy error: /checkout/src/libcore/num/mod.rs:5295: line longer than 100 chars
2019-11-10T17:37:10.5316525Z tidy error: /checkout/src/libcore/num/mod.rs:5296: line longer than 100 chars
2019-11-10T17:37:10.5316587Z tidy error: /checkout/src/libcore/num/mod.rs:5299: line longer than 100 chars
2019-11-10T17:37:10.5316629Z tidy error: /checkout/src/libcore/num/mod.rs:5300: line longer than 100 chars
2019-11-10T17:37:10.5316671Z tidy error: /checkout/src/libcore/num/mod.rs:5301: line longer than 100 chars
2019-11-10T17:37:10.5316730Z tidy error: /checkout/src/libcore/num/mod.rs:5302: line longer than 100 chars
2019-11-10T17:37:10.5316773Z tidy error: /checkout/src/libcore/num/mod.rs:5303: line longer than 100 chars
2019-11-10T17:37:10.5316815Z tidy error: /checkout/src/libcore/num/mod.rs:5304: line longer than 100 chars
2019-11-10T17:37:10.5316874Z tidy error: /checkout/src/libcore/num/mod.rs:5305: line longer than 100 chars
2019-11-10T17:37:10.5316931Z tidy error: /checkout/src/libcore/num/mod.rs:5306: line longer than 100 chars
2019-11-10T17:37:10.5316974Z tidy error: /checkout/src/libcore/num/mod.rs:5307: line longer than 100 chars
2019-11-10T17:37:10.5317032Z tidy error: /checkout/src/libcore/num/mod.rs:5308: line longer than 100 chars
2019-11-10T17:37:10.5317074Z tidy error: /checkout/src/libcore/num/mod.rs:5309: line longer than 100 chars
2019-11-10T17:37:10.5317132Z tidy error: /checkout/src/libcore/num/mod.rs:5312: line longer than 100 chars
2019-11-10T17:37:10.5317175Z tidy error: /checkout/src/libcore/num/mod.rs:5313: line longer than 100 chars
2019-11-10T17:37:10.5317218Z tidy error: /checkout/src/libcore/num/mod.rs:5314: line longer than 100 chars
2019-11-10T17:37:10.5317275Z tidy error: /checkout/src/libcore/num/mod.rs:5315: line longer than 100 chars
2019-11-10T17:37:10.5317317Z tidy error: /checkout/src/libcore/num/mod.rs:5316: line longer than 100 chars
2019-11-10T17:37:10.5317360Z tidy error: /checkout/src/libcore/num/mod.rs:5317: line longer than 100 chars
2019-11-10T17:37:10.5317430Z tidy error: /checkout/src/libcore/num/mod.rs:5318: line longer than 100 chars
2019-11-10T17:37:10.5317648Z tidy error: /checkout/src/libcore/num/mod.rs:5319: line longer than 100 chars
2019-11-10T17:37:10.5317692Z tidy error: /checkout/src/libcore/num/mod.rs:5320: line longer than 100 chars
2019-11-10T17:37:10.5317752Z tidy error: /checkout/src/libcore/num/mod.rs:5321: line longer than 100 chars
2019-11-10T17:37:10.5317796Z tidy error: /checkout/src/libcore/num/mod.rs:5322: line longer than 100 chars
2019-11-10T17:37:12.2724104Z Found 485 error codes
2019-11-10T17:37:12.2724955Z Found 0 error codes with no tests
2019-11-10T17:37:12.2725162Z Done!
2019-11-10T17:37:12.2725292Z some tidy checks failed
2019-11-10T17:37:12.2725292Z some tidy checks failed
2019-11-10T17:37:12.2733181Z 
2019-11-10T17:37:12.2733307Z 
2019-11-10T17:37:12.2734410Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-10T17:37:12.2734737Z 
2019-11-10T17:37:12.2734920Z 
2019-11-10T17:37:12.2738468Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-10T17:37:12.2738743Z Build completed unsuccessfully in 0:01:34
2019-11-10T17:37:12.2738743Z Build completed unsuccessfully in 0:01:34
2019-11-10T17:37:12.2793842Z == clock drift check ==
2019-11-10T17:37:12.2825410Z   local time: Sun Nov 10 17:37:12 UTC 2019
2019-11-10T17:37:12.3173372Z   network time: Sun, 10 Nov 2019 17:37:12 GMT
2019-11-10T17:37:12.3177657Z == end clock drift check ==
2019-11-10T17:37:13.6386906Z 
2019-11-10T17:37:13.6503819Z ##[error]Bash exited with code '1'.
2019-11-10T17:37:13.6532504Z ##[section]Starting: Checkout
2019-11-10T17:37:13.6534425Z ==============================================================================
2019-11-10T17:37:13.6534510Z Task         : Get sources
2019-11-10T17:37:13.6534852Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
