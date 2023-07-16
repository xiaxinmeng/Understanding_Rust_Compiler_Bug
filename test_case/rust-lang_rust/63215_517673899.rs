plain
2019-08-02T11:39:59.8324590Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-02T11:39:59.8517110Z ##[command]git config gc.auto 0
2019-08-02T11:39:59.8598302Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-02T11:39:59.8652494Z ##[command]git config --get-all http.proxy
2019-08-02T11:39:59.8788025Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63215/merge:refs/remotes/pull/63215/merge
---
2019-08-02T11:40:36.2644846Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-02T11:40:36.2644872Z 
2019-08-02T11:40:36.2645069Z   git checkout -b <new-branch-name>
2019-08-02T11:40:36.2645095Z 
2019-08-02T11:40:36.2645137Z HEAD is now at 979ecd08f Merge 57f94237e1ae9be36583270efacaa6556dbc1ce0 into fc3ef9698fa80aa2f4da6208b8295bc8fa48eec5
2019-08-02T11:40:36.2808653Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-02T11:40:36.2811155Z ==============================================================================
2019-08-02T11:40:36.2811222Z Task         : Bash
2019-08-02T11:40:36.2811386Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-02T11:46:27.2362788Z    Compiling serde_json v1.0.40
2019-08-02T11:46:31.6987234Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-02T11:46:40.8269311Z     Finished release [optimized] target(s) in 1m 32s
2019-08-02T11:46:40.8353496Z tidy check
2019-08-02T11:46:41.3022163Z tidy error: /checkout/src/libcore/mem/mod.rs:420: trailing whitespace
2019-08-02T11:46:42.8622560Z some tidy checks failed
2019-08-02T11:46:42.8630533Z 
2019-08-02T11:46:42.8630533Z 
2019-08-02T11:46:42.8632248Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-02T11:46:42.8632900Z 
2019-08-02T11:46:42.8633257Z 
2019-08-02T11:46:42.8633549Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-02T11:46:42.8634285Z Build completed unsuccessfully in 0:01:35
2019-08-02T11:46:42.8634285Z Build completed unsuccessfully in 0:01:35
2019-08-02T11:46:44.2819162Z ##[error]Bash exited with code '1'.
2019-08-02T11:46:44.2855871Z ##[section]Starting: Checkout
2019-08-02T11:46:44.2857891Z ==============================================================================
2019-08-02T11:46:44.2857951Z Task         : Get sources
2019-08-02T11:46:44.2858019Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
