plain
2019-08-10T23:32:28.2422983Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-10T23:32:28.2613982Z ##[command]git config gc.auto 0
2019-08-10T23:32:28.2706874Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-10T23:32:28.2761757Z ##[command]git config --get-all http.proxy
2019-08-10T23:32:28.2899461Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63453/merge:refs/remotes/pull/63453/merge
---
2019-08-10T23:33:01.4592544Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-10T23:33:01.4592787Z 
2019-08-10T23:33:01.4593569Z   git checkout -b <new-branch-name>
2019-08-10T23:33:01.4593784Z 
2019-08-10T23:33:01.4594059Z HEAD is now at f70a77c4b Merge 902d1b08d122e6c99763bf6a9e66754f58cd0efc into 9703ef666123c465f784e294b5b24d6d35a37745
2019-08-10T23:33:01.4750754Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-10T23:33:01.4754249Z ==============================================================================
2019-08-10T23:33:01.4754314Z Task         : Bash
2019-08-10T23:33:01.4754354Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-10T23:38:52.2619085Z    Compiling serde_json v1.0.40
2019-08-10T23:38:56.3816097Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-10T23:39:04.6525395Z     Finished release [optimized] target(s) in 1m 25s
2019-08-10T23:39:04.6596756Z tidy check
2019-08-10T23:39:05.0555623Z tidy error: /checkout/src/librustdoc/fold.rs:109: line longer than 100 chars
2019-08-10T23:39:06.6235180Z some tidy checks failed
2019-08-10T23:39:06.6237998Z 
2019-08-10T23:39:06.6237998Z 
2019-08-10T23:39:06.6239192Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-10T23:39:06.6243024Z 
2019-08-10T23:39:06.6243120Z 
2019-08-10T23:39:06.6259019Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-10T23:39:06.6259106Z Build completed unsuccessfully in 0:01:28
2019-08-10T23:39:06.6259106Z Build completed unsuccessfully in 0:01:28
2019-08-10T23:39:08.2076203Z ##[error]Bash exited with code '1'.
2019-08-10T23:39:08.2128213Z ##[section]Starting: Checkout
2019-08-10T23:39:08.2131095Z ==============================================================================
2019-08-10T23:39:08.2131155Z Task         : Get sources
2019-08-10T23:39:08.2131203Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
