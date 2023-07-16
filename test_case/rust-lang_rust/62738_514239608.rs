plain
2019-07-23T14:21:26.9695381Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-23T14:21:26.9884100Z ##[command]git config gc.auto 0
2019-07-23T14:21:27.0199050Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-23T14:21:27.0250706Z ##[command]git config --get-all http.proxy
2019-07-23T14:21:27.0388086Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62738/merge:refs/remotes/pull/62738/merge
---
2019-07-23T14:22:01.7970652Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-23T14:22:01.7970702Z 
2019-07-23T14:22:01.7970917Z   git checkout -b <new-branch-name>
2019-07-23T14:22:01.7971139Z 
2019-07-23T14:22:01.7971188Z HEAD is now at 79cf6b3f9 Merge e175c874b3d257be60183478512b121891df9225 into 3ebca72a11869f946b31f900faffb75c2bb2473a
2019-07-23T14:22:01.8110506Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-23T14:22:01.8113404Z ==============================================================================
2019-07-23T14:22:01.8113460Z Task         : Bash
2019-07-23T14:22:01.8113505Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-23T14:28:17.4353980Z    Compiling serde_json v1.0.40
2019-07-23T14:28:21.9234004Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-23T14:28:30.7869312Z     Finished release [optimized] target(s) in 1m 32s
2019-07-23T14:28:30.7940627Z tidy check
2019-07-23T14:28:31.6548818Z tidy error: /checkout/src/libstd/sys/cloudabi/mutex.rs:57: trailing whitespace
2019-07-23T14:28:32.6976658Z some tidy checks failed
2019-07-23T14:28:32.6983114Z 
2019-07-23T14:28:32.6983114Z 
2019-07-23T14:28:32.6983974Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-23T14:28:32.6984113Z 
2019-07-23T14:28:32.6984171Z 
2019-07-23T14:28:32.6999612Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-23T14:28:32.6999694Z Build completed unsuccessfully in 0:01:35
2019-07-23T14:28:32.6999694Z Build completed unsuccessfully in 0:01:35
2019-07-23T14:28:33.9959403Z ##[error]Bash exited with code '1'.
2019-07-23T14:28:33.9991535Z ##[section]Starting: Checkout
2019-07-23T14:28:33.9993509Z ==============================================================================
2019-07-23T14:28:33.9993579Z Task         : Get sources
2019-07-23T14:28:33.9993627Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
