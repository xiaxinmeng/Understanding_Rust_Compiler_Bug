plain
2019-07-25T23:31:35.4576537Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-25T23:31:35.4756074Z ##[command]git config gc.auto 0
2019-07-25T23:31:35.4836588Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-25T23:31:35.4895874Z ##[command]git config --get-all http.proxy
2019-07-25T23:31:35.5047156Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62995/merge:refs/remotes/pull/62995/merge
---
2019-07-25T23:32:09.5056160Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-25T23:32:09.5056421Z 
2019-07-25T23:32:09.5056744Z   git checkout -b <new-branch-name>
2019-07-25T23:32:09.5056777Z 
2019-07-25T23:32:09.5056831Z HEAD is now at f842089b1 Merge 2bd2fde71989b5dd01df7a639ab232b78cd68c9d into 890881f8f4c77e8670d4b32104c0325fcfefc90f
2019-07-25T23:32:09.5200783Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-25T23:32:09.5203967Z ==============================================================================
2019-07-25T23:32:09.5204034Z Task         : Bash
2019-07-25T23:32:09.5204089Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-25T23:37:02.7452569Z    Compiling aho-corasick v0.7.3
2019-07-25T23:37:28.4679881Z    Compiling serde_derive v1.0.81
2019-07-25T23:38:02.7723448Z    Compiling serde_json v1.0.40
2019-07-25T23:38:07.0049281Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-25T23:38:07.1009791Z error: cannot find macro `supressible_tidy_error!` in this scope
2019-07-25T23:38:07.1017575Z    --> src/tools/tidy/src/style.rs:221:18
2019-07-25T23:38:07.1022261Z     |
2019-07-25T23:38:07.1027663Z 221 |             0 => supressible_tidy_error!(
2019-07-25T23:38:07.1034506Z     |                  ^^^^^^^^^^^^^^^^^^^^^^ help: you could try the macro: `suppressible_tidy_err`
2019-07-25T23:38:07.1043792Z 
2019-07-25T23:38:07.1050325Z error: cannot find macro `supressible_tidy_error!` in this scope
2019-07-25T23:38:07.1056793Z    --> src/tools/tidy/src/style.rs:228:18
2019-07-25T23:38:07.1061672Z     |
2019-07-25T23:38:07.1061961Z 228 |             n => supressible_tidy_error!(
2019-07-25T23:38:07.1062290Z     |                  ^^^^^^^^^^^^^^^^^^^^^^ help: you could try the macro: `suppressible_tidy_err`
2019-07-25T23:38:07.2472124Z error[E0308]: mismatched types
2019-07-25T23:38:07.2472554Z    --> src/tools/tidy/src/style.rs:220:18
2019-07-25T23:38:07.2472823Z     |
2019-07-25T23:38:07.2472823Z     |
2019-07-25T23:38:07.2473606Z 220 |             n if skip_trailing_newlines => {}
2019-07-25T23:38:07.2474008Z     |                  ^^^^^^^^^^^^^^^^^^^^^^ expected bool, found enum `style::Directive`
2019-07-25T23:38:07.2474578Z     = note: expected type `bool`
2019-07-25T23:38:07.2474838Z                found type `style::Directive`
2019-07-25T23:38:07.2474896Z 
2019-07-25T23:38:07.4487647Z error: aborting due to 3 previous errors
2019-07-25T23:38:07.4487647Z error: aborting due to 3 previous errors
2019-07-25T23:38:07.4487755Z 
2019-07-25T23:38:07.4488086Z For more information about this error, try `rustc --explain E0308`.
2019-07-25T23:38:07.4627993Z error: Could not compile `tidy`.
2019-07-25T23:38:07.4628331Z To learn more, run the command again with --verbose.
2019-07-25T23:38:07.4658867Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/tidy/Cargo.toml" "--message-format" "json"
2019-07-25T23:38:07.4659015Z expected success, got: exit code: 101
2019-07-25T23:38:07.4668109Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-25T23:38:07.4668109Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-25T23:38:07.4668758Z Build completed unsuccessfully in 0:01:21
2019-07-25T23:38:08.8017075Z ##[error]Bash exited with code '1'.
2019-07-25T23:38:08.8050009Z ##[section]Starting: Checkout
2019-07-25T23:38:08.8051732Z ==============================================================================
2019-07-25T23:38:08.8051782Z Task         : Get sources
2019-07-25T23:38:08.8051864Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
