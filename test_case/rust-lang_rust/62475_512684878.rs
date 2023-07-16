plain
2019-07-18T06:14:49.6909790Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-18T06:14:49.7104207Z ##[command]git config gc.auto 0
2019-07-18T06:14:49.7179368Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-18T06:14:49.7225149Z ##[command]git config --get-all http.proxy
2019-07-18T06:14:49.7378374Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62475/merge:refs/remotes/pull/62475/merge
---
2019-07-18T06:15:25.5076408Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-18T06:15:25.5076470Z 
2019-07-18T06:15:25.5076723Z   git checkout -b <new-branch-name>
2019-07-18T06:15:25.5076753Z 
2019-07-18T06:15:25.5076802Z HEAD is now at e949b76ec Merge af4baa08417d08ee127af8e1d1c81825f38318d3 into efb74579150a6ea0ad99a8568aa02a1aa23f6dd5
2019-07-18T06:15:25.5281249Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-18T06:15:25.5284060Z ==============================================================================
2019-07-18T06:15:25.5284117Z Task         : Bash
2019-07-18T06:15:25.5284165Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-18T06:21:56.7994254Z    Compiling serde_json v1.0.40
2019-07-18T06:22:01.2484482Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-18T06:22:10.0843497Z     Finished release [optimized] target(s) in 1m 32s
2019-07-18T06:22:10.0915560Z tidy check
2019-07-18T06:22:10.2958680Z tidy error: /checkout/src/libcore/iter/adapters/zip.rs:4: line longer than 100 chars
2019-07-18T06:22:12.0588959Z some tidy checks failed
2019-07-18T06:22:12.0589146Z 
2019-07-18T06:22:12.0589146Z 
2019-07-18T06:22:12.0611767Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-18T06:22:12.0612463Z 
2019-07-18T06:22:12.0612585Z 
2019-07-18T06:22:12.0613588Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-18T06:22:12.0613760Z Build completed unsuccessfully in 0:01:35
2019-07-18T06:22:12.0613760Z Build completed unsuccessfully in 0:01:35
2019-07-18T06:22:13.3511408Z ##[error]Bash exited with code '1'.
2019-07-18T06:22:13.3547623Z ##[section]Starting: Checkout
2019-07-18T06:22:13.3549269Z ==============================================================================
2019-07-18T06:22:13.3549324Z Task         : Get sources
2019-07-18T06:22:13.3549389Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
