plain
2019-07-29T13:28:49.2377871Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-29T13:28:49.2563713Z ##[command]git config gc.auto 0
2019-07-29T13:28:49.2635012Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-29T13:28:49.2692952Z ##[command]git config --get-all http.proxy
2019-07-29T13:28:49.2823639Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63087/merge:refs/remotes/pull/63087/merge
---
2019-07-29T13:29:23.7967370Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-29T13:29:23.7968107Z 
2019-07-29T13:29:23.7969048Z   git checkout -b <new-branch-name>
2019-07-29T13:29:23.7969892Z 
2019-07-29T13:29:23.7970693Z HEAD is now at d7a6143ae Merge 7d2e8d78371515a560c5bbf1bc66f29ece4cd44a into 8b94e9e9188b65df38a5f1ae723617dc2dfb3155
2019-07-29T13:29:23.8109818Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-29T13:29:23.8112458Z ==============================================================================
2019-07-29T13:29:23.8112506Z Task         : Bash
2019-07-29T13:29:23.8112545Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-29T13:34:31.0860816Z    Compiling aho-corasick v0.7.3
2019-07-29T13:34:55.6144081Z    Compiling serde_derive v1.0.81
2019-07-29T13:35:30.4537442Z    Compiling serde_json v1.0.40
2019-07-29T13:35:34.6990041Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-29T13:35:34.9401561Z error[E0425]: cannot find value `tomlfile` in this scope
2019-07-29T13:35:34.9411069Z   --> src/tools/tidy/src/edition.rs:41:21
2019-07-29T13:35:34.9416635Z    |
2019-07-29T13:35:34.9423666Z 41 |                     tomlfile.display()
2019-07-29T13:35:34.9434601Z 
2019-07-29T13:35:34.9488821Z error: unused import: `std::fs`
2019-07-29T13:35:34.9496817Z  --> src/tools/tidy/src/edition.rs:4:5
2019-07-29T13:35:34.9497033Z   |
---
2019-07-29T13:35:34.9503286Z 
2019-07-29T13:35:35.3213961Z error: aborting due to 2 previous errors
2019-07-29T13:35:35.3214060Z 
2019-07-29T13:35:35.3214294Z For more information about this error, try `rustc --explain E0425`.
2019-07-29T13:35:35.3343399Z error: Could not compile `tidy`.
2019-07-29T13:35:35.3343880Z To learn more, run the command again with --verbose.
2019-07-29T13:35:35.3366340Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/tidy/Cargo.toml" "--message-format" "json"
2019-07-29T13:35:35.3366434Z expected success, got: exit code: 101
2019-07-29T13:35:35.3378097Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-29T13:35:35.3378097Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-29T13:35:35.3378159Z Build completed unsuccessfully in 0:01:20
2019-07-29T13:35:36.5792273Z ##[error]Bash exited with code '1'.
2019-07-29T13:35:36.5822959Z ##[section]Starting: Checkout
2019-07-29T13:35:36.5825684Z ==============================================================================
2019-07-29T13:35:36.5825732Z Task         : Get sources
2019-07-29T13:35:36.5825814Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
