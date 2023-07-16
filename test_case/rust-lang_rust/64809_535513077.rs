plain
2019-09-26T13:35:15.3525571Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-26T13:35:15.3715437Z ##[command]git config gc.auto 0
2019-09-26T13:35:15.3795716Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-26T13:35:15.3862150Z ##[command]git config --get-all http.proxy
2019-09-26T13:35:15.4054899Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64809/merge:refs/remotes/pull/64809/merge
---
2019-09-26T13:42:19.6401777Z    Compiling serde_json v1.0.40
2019-09-26T13:42:21.1726983Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-26T13:42:31.4290540Z     Finished release [optimized] target(s) in 1m 19s
2019-09-26T13:42:31.4360707Z tidy check
2019-09-26T13:42:31.9043977Z tidy error: /checkout/src/librustc/hir/check_attr.rs:138: line longer than 100 chars
2019-09-26T13:42:31.9044140Z tidy error: /checkout/src/librustc/hir/check_attr.rs:168: line longer than 100 chars
2019-09-26T13:42:33.0717758Z some tidy checks failed
2019-09-26T13:42:33.0719556Z 
2019-09-26T13:42:33.0719556Z 
2019-09-26T13:42:33.0720701Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-26T13:42:33.0720869Z 
2019-09-26T13:42:33.0720915Z 
2019-09-26T13:42:33.0732887Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-26T13:42:33.0732944Z Build completed unsuccessfully in 0:01:22
2019-09-26T13:42:33.0732944Z Build completed unsuccessfully in 0:01:22
2019-09-26T13:42:33.0777125Z == clock drift check ==
2019-09-26T13:42:33.0800453Z   local time: Thu Sep 26 13:42:33 UTC 2019
2019-09-26T13:42:33.3412426Z   network time: Thu, 26 Sep 2019 13:42:33 GMT
2019-09-26T13:42:33.3416864Z == end clock drift check ==
2019-09-26T13:42:34.7539413Z ##[error]Bash exited with code '1'.
2019-09-26T13:42:34.7570935Z ##[section]Starting: Checkout
2019-09-26T13:42:34.7573055Z ==============================================================================
2019-09-26T13:42:34.7573295Z Task         : Get sources
2019-09-26T13:42:34.7573349Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
