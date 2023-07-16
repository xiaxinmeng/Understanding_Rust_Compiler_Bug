plain
2019-07-24T02:22:55.9522959Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-24T02:22:55.9720805Z ##[command]git config gc.auto 0
2019-07-24T02:22:55.9789290Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-24T02:22:55.9870737Z ##[command]git config --get-all http.proxy
2019-07-24T02:22:56.8546333Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61515/merge:refs/remotes/pull/61515/merge
---
2019-07-24T02:23:31.9521439Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-24T02:23:31.9521625Z 
2019-07-24T02:23:31.9521990Z   git checkout -b <new-branch-name>
2019-07-24T02:23:31.9522142Z 
2019-07-24T02:23:31.9522292Z HEAD is now at d55d48ffe Merge f0cb1ca8211aec933582dc470b52f95f3402a8b1 into a7f28678bbf4e16893bb6a718e427504167a9494
2019-07-24T02:23:31.9678243Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-24T02:23:31.9681510Z ==============================================================================
2019-07-24T02:23:31.9681572Z Task         : Bash
2019-07-24T02:23:31.9681651Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-24T02:29:53.0535462Z    Compiling serde_json v1.0.40
2019-07-24T02:29:57.5090136Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-24T02:30:06.3564490Z     Finished release [optimized] target(s) in 1m 31s
2019-07-24T02:30:06.3622621Z tidy check
2019-07-24T02:30:06.9189478Z tidy error: /checkout/src/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33.rs:6: line longer than 100 chars
2019-07-24T02:30:06.9189941Z tidy error: /checkout/src/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33.rs:7: line longer than 100 chars
2019-07-24T02:30:06.9190248Z tidy error: /checkout/src/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33.rs:13: line longer than 100 chars
2019-07-24T02:30:06.9190565Z tidy error: /checkout/src/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33.rs:14: line longer than 100 chars
2019-07-24T02:30:06.9191130Z tidy error: /checkout/src/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33.rs:20: line longer than 100 chars
2019-07-24T02:30:06.9191484Z tidy error: /checkout/src/test/ui/const-generics/array-impls/alloc-types-no-impls-length-33.rs:21: line longer than 100 chars
2019-07-24T02:30:08.3762048Z some tidy checks failed
2019-07-24T02:30:08.3771791Z 
2019-07-24T02:30:08.3771791Z 
2019-07-24T02:30:08.3772734Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-24T02:30:08.3772874Z 
2019-07-24T02:30:08.3772902Z 
2019-07-24T02:30:08.3779995Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-24T02:30:08.3780065Z Build completed unsuccessfully in 0:01:35
2019-07-24T02:30:08.3780065Z Build completed unsuccessfully in 0:01:35
2019-07-24T02:30:09.7309125Z ##[error]Bash exited with code '1'.
2019-07-24T02:30:09.7341542Z ##[section]Starting: Checkout
2019-07-24T02:30:09.7343228Z ==============================================================================
2019-07-24T02:30:09.7343303Z Task         : Get sources
2019-07-24T02:30:09.7343351Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
