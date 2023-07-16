plain
2019-08-26T16:09:50.4165698Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-26T16:09:50.4387042Z ##[command]git config gc.auto 0
2019-08-26T16:09:50.4445241Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-26T16:09:50.4496400Z ##[command]git config --get-all http.proxy
2019-08-26T16:09:50.4627476Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63917/merge:refs/remotes/pull/63917/merge
---
2019-08-26T16:10:25.8055222Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-26T16:10:25.8056235Z 
2019-08-26T16:10:25.8057471Z   git checkout -b <new-branch-name>
2019-08-26T16:10:25.8058711Z 
2019-08-26T16:10:25.8059373Z HEAD is now at 72b465179 Merge 330e5f4a03ec5e7eae2a995e92083ba459135916 into 555d7a2fd6165b614cfc01136d8e3f5c465a1582
2019-08-26T16:10:25.8198388Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-26T16:10:25.8201809Z ==============================================================================
2019-08-26T16:10:25.8201867Z Task         : Bash
2019-08-26T16:10:25.8201913Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-26T16:16:42.2261658Z    Compiling serde_json v1.0.40
2019-08-26T16:16:43.9994200Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-26T16:16:54.8296404Z     Finished release [optimized] target(s) in 1m 29s
2019-08-26T16:16:54.8424548Z tidy check
2019-08-26T16:16:55.3223694Z tidy error: /checkout/src/librustc_typeck/check/closure.rs:269: line longer than 100 chars
2019-08-26T16:16:56.7883364Z some tidy checks failed
2019-08-26T16:16:56.7884430Z 
2019-08-26T16:16:56.7884430Z 
2019-08-26T16:16:56.7885639Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-26T16:16:56.7886048Z 
2019-08-26T16:16:56.7886214Z 
2019-08-26T16:16:56.7886479Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-26T16:16:56.7886837Z Build completed unsuccessfully in 0:01:32
2019-08-26T16:16:56.7886837Z Build completed unsuccessfully in 0:01:32
2019-08-26T16:16:56.7929434Z == clock drift check ==
2019-08-26T16:16:56.7946070Z   local time: Mon Aug 26 16:16:56 UTC 2019
2019-08-26T16:16:56.9509774Z   network time: Mon, 26 Aug 2019 16:16:56 GMT
2019-08-26T16:16:56.9510304Z == end clock drift check ==
2019-08-26T16:16:58.3796091Z ##[error]Bash exited with code '1'.
2019-08-26T16:16:58.3847441Z ##[section]Starting: Checkout
2019-08-26T16:16:58.3849669Z ==============================================================================
2019-08-26T16:16:58.3849729Z Task         : Get sources
2019-08-26T16:16:58.3849795Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
