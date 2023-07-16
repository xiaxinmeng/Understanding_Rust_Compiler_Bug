plain
2019-07-26T17:06:20.8768482Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-26T17:06:20.8961584Z ##[command]git config gc.auto 0
2019-07-26T17:06:20.9026563Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-26T17:06:20.9076581Z ##[command]git config --get-all http.proxy
2019-07-26T17:06:20.9230147Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62995/merge:refs/remotes/pull/62995/merge
---
2019-07-26T17:06:55.0899775Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-26T17:06:55.0899824Z 
2019-07-26T17:06:55.0900034Z   git checkout -b <new-branch-name>
2019-07-26T17:06:55.0900064Z 
2019-07-26T17:06:55.0900112Z HEAD is now at d6c93452a Merge b74a390e8225545a04548f0deb36041a37226379 into 1a563362865e6051d4c350544131228e8eff5138
2019-07-26T17:06:55.1029393Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-26T17:06:55.1032096Z ==============================================================================
2019-07-26T17:06:55.1032154Z Task         : Bash
2019-07-26T17:06:55.1032201Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-26T17:13:08.1392020Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-26T17:13:08.5416535Z error: unused variable: `n`
2019-07-26T17:13:08.5417795Z    --> src/tools/tidy/src/style.rs:232:13
2019-07-26T17:13:08.5418414Z     |
2019-07-26T17:13:08.5419101Z 232 |             n if skip_trailing_newlines.is_ignore() => {}
2019-07-26T17:13:08.5419860Z     |             ^ help: consider prefixing with an underscore: `_n`
2019-07-26T17:13:08.5421119Z     = note: `-D unused-variables` implied by `-D warnings`
2019-07-26T17:13:08.5421420Z 
2019-07-26T17:13:08.7872621Z error: aborting due to previous error
2019-07-26T17:13:08.7873370Z 
2019-07-26T17:13:08.7873370Z 
2019-07-26T17:13:08.8044740Z error: Could not compile `tidy`.
2019-07-26T17:13:08.8045219Z To learn more, run the command again with --verbose.
2019-07-26T17:13:08.8098455Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/tidy/Cargo.toml" "--message-format" "json"
2019-07-26T17:13:08.8101447Z expected success, got: exit code: 101
2019-07-26T17:13:08.8107177Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-26T17:13:08.8107177Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-26T17:13:08.8107593Z Build completed unsuccessfully in 0:01:25
2019-07-26T17:13:10.0846519Z ##[error]Bash exited with code '1'.
2019-07-26T17:13:10.0876902Z ##[section]Starting: Checkout
2019-07-26T17:13:10.0878605Z ==============================================================================
2019-07-26T17:13:10.0878676Z Task         : Get sources
2019-07-26T17:13:10.0878725Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
