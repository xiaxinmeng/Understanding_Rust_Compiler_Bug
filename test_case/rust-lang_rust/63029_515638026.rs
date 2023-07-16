plain
2019-07-27T00:31:22.3603572Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-27T00:31:22.3835680Z ##[command]git config gc.auto 0
2019-07-27T00:31:22.3924836Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-27T00:31:22.3984989Z ##[command]git config --get-all http.proxy
2019-07-27T00:31:22.4121795Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63029/merge:refs/remotes/pull/63029/merge
---
2019-07-27T00:31:56.7163890Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-27T00:31:56.7163922Z 
2019-07-27T00:31:56.7164129Z   git checkout -b <new-branch-name>
2019-07-27T00:31:56.7164157Z 
2019-07-27T00:31:56.7164223Z HEAD is now at 7a142a849 Merge 6050b076f2853f7fa4c2fe2f1d9a7e9d8121ba6e into c43753f910aae000f8bcb0a502407ea332afc74b
2019-07-27T00:31:56.7308023Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-27T00:31:56.7310894Z ==============================================================================
2019-07-27T00:31:56.7310948Z Task         : Bash
2019-07-27T00:31:56.7311010Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-27T00:39:31.9370797Z    Compiling serde_json v1.0.40
2019-07-27T00:39:36.5468936Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-27T00:39:45.5465175Z     Finished release [optimized] target(s) in 1m 35s
2019-07-27T00:39:45.5549857Z tidy check
2019-07-27T00:39:46.0320554Z tidy error: /checkout/src/test/ui/lexer-crlf-line-endings-string-literal-doc-comment.rs: ignoring CR characters unnecessarily
2019-07-27T00:39:47.5830867Z some tidy checks failed
2019-07-27T00:39:47.5831032Z 
2019-07-27T00:39:47.5831032Z 
2019-07-27T00:39:47.5832220Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-27T00:39:47.5832390Z 
2019-07-27T00:39:47.5832416Z 
2019-07-27T00:39:47.5840398Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-27T00:39:47.5878456Z Build completed unsuccessfully in 0:01:38
2019-07-27T00:39:47.5878456Z Build completed unsuccessfully in 0:01:38
2019-07-27T00:39:48.9872633Z ##[error]Bash exited with code '1'.
2019-07-27T00:39:48.9907189Z ##[section]Starting: Checkout
2019-07-27T00:39:48.9908647Z ==============================================================================
2019-07-27T00:39:48.9908695Z Task         : Get sources
2019-07-27T00:39:48.9908754Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
