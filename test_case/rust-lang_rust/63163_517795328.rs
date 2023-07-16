plain
2019-08-02T17:47:05.3347477Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-02T17:47:05.3582129Z ##[command]git config gc.auto 0
2019-08-02T17:47:05.3634840Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-02T17:47:05.3679553Z ##[command]git config --get-all http.proxy
2019-08-02T17:47:05.3823154Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63163/merge:refs/remotes/pull/63163/merge
---
2019-08-02T17:47:40.4690006Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-02T17:47:40.4690290Z 
2019-08-02T17:47:40.4690771Z   git checkout -b <new-branch-name>
2019-08-02T17:47:40.4691031Z 
2019-08-02T17:47:40.4691292Z HEAD is now at b19fe750b Merge e64493e2d4f5e1e60c2599b0eea165a5bde0fdc6 into 1df512fcaeaf17639c5d28a3045814d6f7a7db97
2019-08-02T17:47:40.4842602Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-02T17:47:40.4845030Z ==============================================================================
2019-08-02T17:47:40.4845080Z Task         : Bash
2019-08-02T17:47:40.4845135Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-02T17:53:47.8899608Z    Compiling serde_json v1.0.40
2019-08-02T17:53:51.9825354Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-02T17:54:00.4293332Z     Finished release [optimized] target(s) in 1m 26s
2019-08-02T17:54:00.4362421Z tidy check
2019-08-02T17:54:00.5527277Z tidy error: /checkout/src/librustc_lint/unused.rs:390: line longer than 100 chars
2019-08-02T17:54:00.5528201Z tidy error: /checkout/src/librustc_lint/unused.rs:411: line longer than 100 chars
2019-08-02T17:54:00.6041634Z tidy error: /checkout/src/test/ui/lint/used_parens_remove_json_suggestion.rs: missing trailing newline
2019-08-02T17:54:02.3609673Z some tidy checks failed
2019-08-02T17:54:02.3609846Z 
2019-08-02T17:54:02.3609846Z 
2019-08-02T17:54:02.3610694Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-02T17:54:02.3610817Z 
2019-08-02T17:54:02.3610844Z 
2019-08-02T17:54:02.3616634Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-02T17:54:02.3616877Z Build completed unsuccessfully in 0:01:29
2019-08-02T17:54:02.3616877Z Build completed unsuccessfully in 0:01:29
2019-08-02T17:54:03.6936981Z ##[error]Bash exited with code '1'.
2019-08-02T17:54:03.6967300Z ##[section]Starting: Checkout
2019-08-02T17:54:03.6968939Z ==============================================================================
2019-08-02T17:54:03.6968993Z Task         : Get sources
2019-08-02T17:54:03.6969069Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
