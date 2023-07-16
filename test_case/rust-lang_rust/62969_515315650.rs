plain
2019-07-26T05:23:04.1465692Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-26T05:23:04.1636586Z ##[command]git config gc.auto 0
2019-07-26T05:23:04.1705728Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-26T05:23:04.1757752Z ##[command]git config --get-all http.proxy
2019-07-26T05:23:04.1887527Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62969/merge:refs/remotes/pull/62969/merge
---
2019-07-26T05:23:37.3471497Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-26T05:23:37.3471554Z 
2019-07-26T05:23:37.3471910Z   git checkout -b <new-branch-name>
2019-07-26T05:23:37.3471935Z 
2019-07-26T05:23:37.3471977Z HEAD is now at c92f7d53f Merge 38d01db845699f15b573bad633b960012f0d1b64 into 18630677cf6c7ac50e6786c504b35bc09501dbe2
2019-07-26T05:23:37.3627393Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-26T05:23:37.3630913Z ==============================================================================
2019-07-26T05:23:37.3630993Z Task         : Bash
2019-07-26T05:23:37.3631040Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-26T05:30:58.0362918Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-07-26T05:31:13.8427137Z error: unreachable expression
2019-07-26T05:31:13.8427705Z    --> src/librustc/mir/interpret/error.rs:325:33
2019-07-26T05:31:13.8427944Z     |
2019-07-26T05:31:13.8428256Z 325 | #[derive(Clone, RustcEncodable, RustcDecodable, HashStable)]
2019-07-26T05:31:13.8428816Z     |
2019-07-26T05:31:13.8429475Z     = note: `-D unreachable-code` implied by `-D warnings`
2019-07-26T05:31:13.8429542Z 
2019-07-26T05:31:13.8436735Z error: unreachable expression
2019-07-26T05:31:13.8436735Z error: unreachable expression
2019-07-26T05:31:13.8437551Z    --> src/librustc/mir/interpret/error.rs:329:33
2019-07-26T05:31:13.8438629Z     |
2019-07-26T05:31:13.8439713Z 329 | #[derive(Clone, RustcEncodable, RustcDecodable, HashStable)]
2019-07-26T05:31:13.8440604Z 
2019-07-26T05:31:13.8457934Z error: unreachable expression
2019-07-26T05:31:13.8458579Z    --> src/librustc/mir/interpret/error.rs:333:33
2019-07-26T05:31:13.8459113Z     |
2019-07-26T05:31:13.8459113Z     |
2019-07-26T05:31:13.8459855Z 333 | #[derive(Clone, RustcEncodable, RustcDecodable, HashStable)]
2019-07-26T05:31:13.8460576Z 
2019-07-26T05:31:22.5972626Z error: aborting due to 3 previous errors
2019-07-26T05:31:22.5973721Z 
2019-07-26T05:31:22.7773710Z error: Could not compile `rustc`.
2019-07-26T05:31:22.7773710Z error: Could not compile `rustc`.
2019-07-26T05:31:22.7774460Z 
2019-07-26T05:31:22.7774982Z To learn more, run the command again with --verbose.
2019-07-26T05:31:22.7795319Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-26T05:31:22.7804416Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-26T05:31:22.7804533Z Build completed unsuccessfully in 0:04:50
2019-07-26T05:31:22.7804533Z Build completed unsuccessfully in 0:04:50
2019-07-26T05:31:23.5367898Z ##[error]Bash exited with code '1'.
2019-07-26T05:31:23.5396174Z ##[section]Starting: Checkout
2019-07-26T05:31:23.5398229Z ==============================================================================
2019-07-26T05:31:23.5398304Z Task         : Get sources
2019-07-26T05:31:23.5398356Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
