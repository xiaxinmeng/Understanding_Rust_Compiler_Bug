plain
2019-07-26T18:30:02.0736499Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-26T18:30:02.0936844Z ##[command]git config gc.auto 0
2019-07-26T18:30:02.1019905Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-26T18:30:02.1070970Z ##[command]git config --get-all http.proxy
2019-07-26T18:30:02.8357734Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62995/merge:refs/remotes/pull/62995/merge
---
2019-07-26T18:30:38.8058945Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-26T18:30:38.8059184Z 
2019-07-26T18:30:38.8059632Z   git checkout -b <new-branch-name>
2019-07-26T18:30:38.8059868Z 
2019-07-26T18:30:38.8060098Z HEAD is now at e79eb804e Merge 3785232b3640a84b58ee5a1f0e2f57479d18684f into 1a563362865e6051d4c350544131228e8eff5138
2019-07-26T18:30:38.8196375Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-26T18:30:38.8199033Z ==============================================================================
2019-07-26T18:30:38.8199100Z Task         : Bash
2019-07-26T18:30:38.8199140Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-26T18:35:34.5340395Z    Compiling aho-corasick v0.7.3
2019-07-26T18:36:00.5809080Z    Compiling serde_derive v1.0.81
2019-07-26T18:36:36.0138944Z    Compiling serde_json v1.0.40
2019-07-26T18:36:40.4407535Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-26T18:36:41.2393994Z error: method is never used: `is_ignore`
2019-07-26T18:36:41.2394583Z    --> src/tools/tidy/src/style.rs:111:5
2019-07-26T18:36:41.2395165Z     |
2019-07-26T18:36:41.2395464Z 111 |     fn is_ignore(&self) -> bool {
2019-07-26T18:36:41.2395989Z     |
2019-07-26T18:36:41.2396277Z     = note: `-D dead-code` implied by `-D warnings`
2019-07-26T18:36:41.2396317Z 
2019-07-26T18:36:41.2534667Z error: aborting due to previous error
2019-07-26T18:36:41.2534667Z error: aborting due to previous error
2019-07-26T18:36:41.2534801Z 
2019-07-26T18:36:41.2706227Z error: Could not compile `tidy`.
2019-07-26T18:36:41.2706704Z To learn more, run the command again with --verbose.
2019-07-26T18:36:41.2733956Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/tidy/Cargo.toml" "--message-format" "json"
2019-07-26T18:36:41.2734089Z expected success, got: exit code: 101
2019-07-26T18:36:41.2745187Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-26T18:36:41.2745187Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-26T18:36:41.2745430Z Build completed unsuccessfully in 0:01:24
2019-07-26T18:36:42.5864752Z ##[error]Bash exited with code '1'.
2019-07-26T18:36:42.5902551Z ##[section]Starting: Checkout
2019-07-26T18:36:42.5904229Z ==============================================================================
2019-07-26T18:36:42.5904282Z Task         : Get sources
2019-07-26T18:36:42.5904351Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
