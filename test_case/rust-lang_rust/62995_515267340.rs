plain
2019-07-26T00:32:07.3526432Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-26T00:32:07.3725339Z ##[command]git config gc.auto 0
2019-07-26T00:32:07.3804508Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-26T00:32:07.3862187Z ##[command]git config --get-all http.proxy
2019-07-26T00:32:07.4017027Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62995/merge:refs/remotes/pull/62995/merge
---
2019-07-26T00:32:42.7003979Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-26T00:32:42.7004031Z 
2019-07-26T00:32:42.7004302Z   git checkout -b <new-branch-name>
2019-07-26T00:32:42.7004336Z 
2019-07-26T00:32:42.7004405Z HEAD is now at 4e3c8977d Merge 7ed4b41d3b8597d54ad7cd1b4cdafebf935dda7b into 890881f8f4c77e8670d4b32104c0325fcfefc90f
2019-07-26T00:32:42.7136555Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-26T00:32:42.7139493Z ==============================================================================
2019-07-26T00:32:42.7139553Z Task         : Bash
2019-07-26T00:32:42.7139617Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-26T00:38:09.8992301Z    Compiling aho-corasick v0.7.3
2019-07-26T00:38:36.2313724Z    Compiling serde_derive v1.0.81
2019-07-26T00:39:12.6049620Z    Compiling serde_json v1.0.40
2019-07-26T00:39:17.2090522Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-26T00:39:17.3137987Z error: cannot find macro `supressible_tidy_err!` in this scope
2019-07-26T00:39:17.3149330Z    --> src/tools/tidy/src/style.rs:230:18
2019-07-26T00:39:17.3154030Z     |
2019-07-26T00:39:17.3158593Z 230 |             0 => supressible_tidy_err!(
2019-07-26T00:39:17.3164466Z     |                  ^^^^^^^^^^^^^^^^^^^^ help: you could try the macro: `suppressible_tidy_err`
2019-07-26T00:39:17.3164844Z 
2019-07-26T00:39:17.3172498Z error: cannot find macro `supressible_tidy_err!` in this scope
2019-07-26T00:39:17.3178588Z    --> src/tools/tidy/src/style.rs:237:18
2019-07-26T00:39:17.3183963Z     |
2019-07-26T00:39:17.3189258Z 237 |             n => supressible_tidy_err!(
2019-07-26T00:39:17.3194121Z     |                  ^^^^^^^^^^^^^^^^^^^^ help: you could try the macro: `suppressible_tidy_err`
2019-07-26T00:39:17.7106896Z error: aborting due to 2 previous errors
2019-07-26T00:39:17.7108171Z 
2019-07-26T00:39:17.7108171Z 
2019-07-26T00:39:17.7267684Z error: Could not compile `tidy`.
2019-07-26T00:39:17.7269243Z To learn more, run the command again with --verbose.
2019-07-26T00:39:17.7294801Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/tidy/Cargo.toml" "--message-format" "json"
2019-07-26T00:39:17.7294929Z expected success, got: exit code: 101
2019-07-26T00:39:17.7306650Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-26T00:39:17.7306650Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-26T00:39:17.7307021Z Build completed unsuccessfully in 0:01:25
2019-07-26T00:39:19.0713446Z ##[error]Bash exited with code '1'.
2019-07-26T00:39:19.0749094Z ##[section]Starting: Checkout
2019-07-26T00:39:19.0750870Z ==============================================================================
2019-07-26T00:39:19.0750932Z Task         : Get sources
2019-07-26T00:39:19.0750986Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
