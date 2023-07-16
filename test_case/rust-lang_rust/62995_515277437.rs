plain
2019-07-26T01:31:47.2764908Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-26T01:31:47.2967425Z ##[command]git config gc.auto 0
2019-07-26T01:31:47.3054114Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-26T01:31:47.3124729Z ##[command]git config --get-all http.proxy
2019-07-26T01:31:47.3270783Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62995/merge:refs/remotes/pull/62995/merge
---
2019-07-26T01:32:21.2275493Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-26T01:32:21.2275544Z 
2019-07-26T01:32:21.2275813Z   git checkout -b <new-branch-name>
2019-07-26T01:32:21.2275844Z 
2019-07-26T01:32:21.2275897Z HEAD is now at f050926f6 Merge b2aa0956c58ef58b5ffc82d396b544fb7af8b310 into 890881f8f4c77e8670d4b32104c0325fcfefc90f
2019-07-26T01:32:21.2431515Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-26T01:32:21.2434216Z ==============================================================================
2019-07-26T01:32:21.2434292Z Task         : Bash
2019-07-26T01:32:21.2434339Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-26T01:37:48.8636114Z    Compiling aho-corasick v0.7.3
2019-07-26T01:38:15.7257385Z    Compiling serde_derive v1.0.81
2019-07-26T01:38:52.2717947Z    Compiling serde_json v1.0.40
2019-07-26T01:38:56.8094744Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-26T01:38:56.8857020Z error: no rules expected the token `,`
2019-07-26T01:38:56.8866406Z    --> src/tools/tidy/src/style.rs:233:73
2019-07-26T01:38:56.8869865Z     |
2019-07-26T01:38:56.8874067Z 132 | macro_rules! suppressible_tidy_err {
2019-07-26T01:38:56.8878415Z     | ---------------------------------- when calling this macro
2019-07-26T01:38:56.8878758Z ...
2019-07-26T01:38:56.8884771Z 233 |                 &format!("{}: missing trailing newline", file.display()),
2019-07-26T01:38:56.8889973Z     |                                                                         ^ no rules expected this token in macro call
2019-07-26T01:38:56.8892016Z 
2019-07-26T01:38:56.8897956Z error: no rules expected the token `,`
2019-07-26T01:38:56.8904134Z    --> src/tools/tidy/src/style.rs:239:83
2019-07-26T01:38:56.8909214Z     |
2019-07-26T01:38:56.8911482Z 132 | macro_rules! suppressible_tidy_err {
2019-07-26T01:38:56.8913137Z     | ---------------------------------- when calling this macro
2019-07-26T01:38:56.8914701Z ...
2019-07-26T01:38:56.8917034Z 239 |                 &format!("{}: too many trailing newlines ({})", file.display(), n),
2019-07-26T01:38:56.8917429Z     |                                                                                   ^ no rules expected this token in macro call
2019-07-26T01:38:57.3109202Z error: aborting due to 2 previous errors
2019-07-26T01:38:57.3110136Z 
2019-07-26T01:38:57.3110136Z 
2019-07-26T01:38:57.3272596Z error: Could not compile `tidy`.
2019-07-26T01:38:57.3274588Z To learn more, run the command again with --verbose.
2019-07-26T01:38:57.3305699Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/tidy/Cargo.toml" "--message-format" "json"
2019-07-26T01:38:57.3306149Z expected success, got: exit code: 101
2019-07-26T01:38:57.3316161Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-26T01:38:57.3316161Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-26T01:38:57.3316442Z Build completed unsuccessfully in 0:01:26
2019-07-26T01:38:58.6212717Z ##[error]Bash exited with code '1'.
2019-07-26T01:38:58.6248678Z ##[section]Starting: Checkout
2019-07-26T01:38:58.6250447Z ==============================================================================
2019-07-26T01:38:58.6250543Z Task         : Get sources
2019-07-26T01:38:58.6250597Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
