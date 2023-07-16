plain
2019-09-27T13:20:53.3281238Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-27T13:20:53.3471597Z ##[command]git config gc.auto 0
2019-09-27T13:20:54.1281247Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-27T13:20:54.1295706Z ##[command]git config --get-all http.proxy
2019-09-27T13:20:54.1303564Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64841/merge:refs/remotes/pull/64841/merge
---
2019-09-27T13:27:49.3376822Z    Compiling serde_json v1.0.40
2019-09-27T13:27:51.2821188Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-27T13:28:02.8011993Z     Finished release [optimized] target(s) in 1m 33s
2019-09-27T13:28:02.8125587Z tidy check
2019-09-27T13:28:03.2751102Z tidy error: /checkout/src/librustc_mir/transform/simplify.rs:280: line longer than 100 chars
2019-09-27T13:28:03.6545743Z tidy error: /checkout/src/librustc/mir/visit.rs:850: TODO is deprecated; use FIXME
2019-09-27T13:28:03.6577318Z tidy error: /checkout/src/librustc/mir/mod.rs:216: line longer than 100 chars
2019-09-27T13:28:03.6579212Z tidy error: /checkout/src/librustc/mir/mod.rs:278: TODO is deprecated; use FIXME
2019-09-27T13:28:04.9639765Z some tidy checks failed
2019-09-27T13:28:04.9645447Z 
2019-09-27T13:28:04.9645447Z 
2019-09-27T13:28:04.9646685Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-27T13:28:04.9646928Z 
2019-09-27T13:28:04.9647316Z 
2019-09-27T13:28:04.9660500Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-27T13:28:04.9660581Z Build completed unsuccessfully in 0:01:36
2019-09-27T13:28:04.9660581Z Build completed unsuccessfully in 0:01:36
2019-09-27T13:28:04.9711283Z == clock drift check ==
2019-09-27T13:28:04.9768355Z   local time: Fri Sep 27 13:28:04 UTC 2019
2019-09-27T13:28:05.1262633Z   network time: Fri, 27 Sep 2019 13:28:05 GMT
2019-09-27T13:28:05.1268739Z == end clock drift check ==
2019-09-27T13:28:06.6398448Z ##[error]Bash exited with code '1'.
2019-09-27T13:28:06.6445118Z ##[section]Starting: Checkout
2019-09-27T13:28:06.6447118Z ==============================================================================
2019-09-27T13:28:06.6447199Z Task         : Get sources
2019-09-27T13:28:06.6447248Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
