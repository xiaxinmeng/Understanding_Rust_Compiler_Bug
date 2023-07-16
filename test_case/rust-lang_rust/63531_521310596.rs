plain
2019-08-14T15:45:17.9245251Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-14T15:45:17.9433573Z ##[command]git config gc.auto 0
2019-08-14T15:45:17.9517977Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-14T15:45:17.9572922Z ##[command]git config --get-all http.proxy
2019-08-14T15:45:17.9714037Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63531/merge:refs/remotes/pull/63531/merge
---
2019-08-14T15:45:53.2665637Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-14T15:45:53.2666024Z 
2019-08-14T15:45:53.2666351Z   git checkout -b <new-branch-name>
2019-08-14T15:45:53.2666383Z 
2019-08-14T15:45:53.2666433Z HEAD is now at d3e305fa8 Merge fbe64f42f5617ab00e4d38285bd81d62c7fc5af0 into c43d03a19f326f4a323569328cc501e86eb6d22e
2019-08-14T15:45:53.2847109Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-14T15:45:53.2850505Z ==============================================================================
2019-08-14T15:45:53.2850565Z Task         : Bash
2019-08-14T15:45:53.2850627Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-14T15:51:49.8453673Z    Compiling serde_json v1.0.40
2019-08-14T15:51:54.1201617Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-14T15:52:02.7147136Z     Finished release [optimized] target(s) in 1m 28s
2019-08-14T15:52:02.7225025Z tidy check
2019-08-14T15:52:03.5105371Z tidy error: /checkout/src/librustc/ty/layout.rs:775: line longer than 100 chars
2019-08-14T15:52:04.6673822Z some tidy checks failed
2019-08-14T15:52:04.6674129Z 
2019-08-14T15:52:04.6674129Z 
2019-08-14T15:52:04.6681022Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-14T15:52:04.6681709Z 
2019-08-14T15:52:04.6681742Z 
2019-08-14T15:52:04.6691166Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-14T15:52:04.6691243Z Build completed unsuccessfully in 0:01:31
2019-08-14T15:52:04.6691243Z Build completed unsuccessfully in 0:01:31
2019-08-14T15:52:04.6746462Z == clock drift check ==
2019-08-14T15:52:04.6755815Z   local time: Wed Aug 14 15:52:04 UTC 2019
2019-08-14T15:52:04.7024085Z   network time: Wed, 14 Aug 2019 15:52:04 GMT
2019-08-14T15:52:04.7032104Z == end clock drift check ==
2019-08-14T15:52:05.9853004Z ##[error]Bash exited with code '1'.
2019-08-14T15:52:05.9883207Z ##[section]Starting: Checkout
2019-08-14T15:52:05.9884793Z ==============================================================================
2019-08-14T15:52:05.9884842Z Task         : Get sources
2019-08-14T15:52:05.9884898Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
