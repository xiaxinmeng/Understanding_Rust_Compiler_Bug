plain
2019-07-26T14:41:23.7849095Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-26T14:41:23.8051411Z ##[command]git config gc.auto 0
2019-07-26T14:41:23.8111544Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-26T14:41:23.8167807Z ##[command]git config --get-all http.proxy
2019-07-26T14:41:23.8312684Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62995/merge:refs/remotes/pull/62995/merge
---
2019-07-26T14:41:59.1780576Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-26T14:41:59.1780607Z 
2019-07-26T14:41:59.1780817Z   git checkout -b <new-branch-name>
2019-07-26T14:41:59.1780864Z 
2019-07-26T14:41:59.1780915Z HEAD is now at 90eeb0be4 Merge 10738d81a2e779a936ac6e91cdbc31cbb507c6ba into 1a563362865e6051d4c350544131228e8eff5138
2019-07-26T14:41:59.1924900Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-26T14:41:59.1927738Z ==============================================================================
2019-07-26T14:41:59.1928067Z Task         : Bash
2019-07-26T14:41:59.1928120Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-26T14:47:03.6702290Z    Compiling aho-corasick v0.7.3
2019-07-26T14:47:30.6362639Z    Compiling serde_derive v1.0.81
2019-07-26T14:48:07.6759845Z    Compiling serde_json v1.0.40
2019-07-26T14:48:12.1419845Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-26T14:48:12.4210872Z error[E0618]: expected function, found `&mut bool`
2019-07-26T14:48:12.4211242Z    --> src/tools/tidy/src/style.rs:135:13
2019-07-26T14:48:12.4211498Z     |
2019-07-26T14:48:12.4211814Z 132 | / macro_rules! suppressible_tidy_err {
2019-07-26T14:48:12.4212169Z 133 | |     ($err:ident, $skip:ident, $msg:expr) => {
2019-07-26T14:48:12.4212688Z 134 | |         if let Directive::Deny = $skip {
2019-07-26T14:48:12.4212982Z 135 | |             $err($msg);
2019-07-26T14:48:12.4213783Z     | |             ^^^^     - call expression requires function
2019-07-26T14:48:12.4214563Z 139 | |     };
2019-07-26T14:48:12.4214845Z 140 | | }
2019-07-26T14:48:12.4214845Z 140 | | }
2019-07-26T14:48:12.4215168Z     | |_- in this expansion of `suppressible_tidy_err!`
2019-07-26T14:48:12.4215435Z 141 | 
2019-07-26T14:48:12.4215725Z 142 |   pub fn check(path: &Path, bad: &mut bool) {
2019-07-26T14:48:12.4216603Z     |                             --- `&mut bool` defined here
2019-07-26T14:48:12.4216898Z ...
2019-07-26T14:48:12.4217193Z 230 |               0 => suppressible_tidy_err!(
2019-07-26T14:48:12.4217783Z 231 | |                 bad,
2019-07-26T14:48:12.4218313Z 232 | |                 skip_trailing_newlines,
2019-07-26T14:48:12.4218313Z 232 | |                 skip_trailing_newlines,
2019-07-26T14:48:12.4218688Z 233 | |                 &format!("{}: missing trailing newline", file.display())
2019-07-26T14:48:12.4219309Z     | |_____________- in this macro invocation
2019-07-26T14:48:12.4219369Z 
2019-07-26T14:48:12.4219369Z 
2019-07-26T14:48:12.4219629Z error[E0618]: expected function, found `&mut bool`
2019-07-26T14:48:12.4219888Z    --> src/tools/tidy/src/style.rs:135:13
2019-07-26T14:48:12.4220126Z     |
2019-07-26T14:48:12.4220445Z 132 | / macro_rules! suppressible_tidy_err {
2019-07-26T14:48:12.4220769Z 133 | |     ($err:ident, $skip:ident, $msg:expr) => {
2019-07-26T14:48:12.4221112Z 134 | |         if let Directive::Deny = $skip {
2019-07-26T14:48:12.4221424Z 135 | |             $err($msg);
2019-07-26T14:48:12.4221987Z     | |             ^^^^     - call expression requires function
2019-07-26T14:48:12.4222713Z 139 | |     };
2019-07-26T14:48:12.4223034Z 140 | | }
2019-07-26T14:48:12.4223034Z 140 | | }
2019-07-26T14:48:12.4223356Z     | |_- in this expansion of `suppressible_tidy_err!`
2019-07-26T14:48:12.4223620Z 141 | 
2019-07-26T14:48:12.4224016Z 142 |   pub fn check(path: &Path, bad: &mut bool) {
2019-07-26T14:48:12.4224390Z     |                             --- `&mut bool` defined here
2019-07-26T14:48:12.4224626Z ...
2019-07-26T14:48:12.4224914Z 236 |               n => suppressible_tidy_err!(
2019-07-26T14:48:12.4225513Z 237 | |                 bad,
2019-07-26T14:48:12.4225832Z 238 | |                 skip_trailing_newlines,
2019-07-26T14:48:12.4225832Z 238 | |                 skip_trailing_newlines,
2019-07-26T14:48:12.4226406Z 239 | |                 &format!("{}: too many trailing newlines ({})", file.display(), n)
2019-07-26T14:48:12.4227081Z     | |_____________- in this macro invocation
2019-07-26T14:48:12.4227148Z 
2019-07-26T14:48:12.6392429Z error: aborting due to 2 previous errors
2019-07-26T14:48:12.6392746Z 
2019-07-26T14:48:12.6392746Z 
2019-07-26T14:48:12.6393330Z For more information about this error, try `rustc --explain E0618`.
2019-07-26T14:48:12.6532960Z error: Could not compile `tidy`.
2019-07-26T14:48:12.6533343Z To learn more, run the command again with --verbose.
2019-07-26T14:48:12.6601125Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/tidy/Cargo.toml" "--message-format" "json"
2019-07-26T14:48:12.6601259Z expected success, got: exit code: 101
2019-07-26T14:48:12.6607790Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-26T14:48:12.6607790Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-26T14:48:12.6607895Z Build completed unsuccessfully in 0:01:26
2019-07-26T14:48:13.9464299Z ##[error]Bash exited with code '1'.
2019-07-26T14:48:13.9548862Z ##[section]Starting: Checkout
2019-07-26T14:48:13.9550747Z ==============================================================================
2019-07-26T14:48:13.9550805Z Task         : Get sources
2019-07-26T14:48:13.9550852Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
