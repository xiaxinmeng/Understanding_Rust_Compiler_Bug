plain
2019-07-24T22:38:57.5067024Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-24T22:38:57.5270949Z ##[command]git config gc.auto 0
2019-07-24T22:38:57.5350759Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-24T22:38:57.5416317Z ##[command]git config --get-all http.proxy
2019-07-24T22:38:57.5559179Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62946/merge:refs/remotes/pull/62946/merge
---
2019-07-24T22:39:32.4586493Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-24T22:39:32.4586773Z 
2019-07-24T22:39:32.4587297Z   git checkout -b <new-branch-name>
2019-07-24T22:39:32.4587559Z 
2019-07-24T22:39:32.4588296Z HEAD is now at 52b1cf241 Merge 3cd4079a9a4a8aa4c8bf566e8c86c528da2810bc into 03f19f7ff128a3b01eeab3f87f04cce22883f006
2019-07-24T22:39:32.4716433Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-24T22:39:32.4720392Z ==============================================================================
2019-07-24T22:39:32.4720451Z Task         : Bash
2019-07-24T22:39:32.4720497Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-24T22:45:41.1828395Z    Compiling serde_json v1.0.40
2019-07-24T22:45:45.4270369Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-24T22:45:54.0991167Z     Finished release [optimized] target(s) in 1m 29s
2019-07-24T22:45:54.1065119Z tidy check
2019-07-24T22:45:54.3852882Z tidy error: /checkout/src/librustc_mir/interpret/cast.rs:146: line longer than 100 chars
2019-07-24T22:45:54.5488119Z tidy error: /checkout/src/test/ui/consts/const-eval/match-test-ptr-null.rs: leading newline
2019-07-24T22:45:55.9199614Z some tidy checks failed
2019-07-24T22:45:55.9205777Z 
2019-07-24T22:45:55.9205777Z 
2019-07-24T22:45:55.9206609Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-24T22:45:55.9206786Z 
2019-07-24T22:45:55.9206812Z 
2019-07-24T22:45:55.9209616Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-24T22:45:55.9209683Z Build completed unsuccessfully in 0:01:32
2019-07-24T22:45:55.9209683Z Build completed unsuccessfully in 0:01:32
2019-07-24T22:45:57.3422281Z ##[error]Bash exited with code '1'.
2019-07-24T22:45:57.3452364Z ##[section]Starting: Checkout
2019-07-24T22:45:57.3453990Z ==============================================================================
2019-07-24T22:45:57.3454043Z Task         : Get sources
2019-07-24T22:45:57.3454108Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
