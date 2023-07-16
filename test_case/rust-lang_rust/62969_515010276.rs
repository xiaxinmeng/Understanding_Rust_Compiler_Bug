plain
2019-07-25T11:30:57.4808714Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-25T11:30:57.4996339Z ##[command]git config gc.auto 0
2019-07-25T11:30:57.5065820Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-25T11:30:57.5112719Z ##[command]git config --get-all http.proxy
2019-07-25T11:30:57.5245553Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62969/merge:refs/remotes/pull/62969/merge
---
2019-07-25T11:31:31.5974737Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-25T11:31:31.5974768Z 
2019-07-25T11:31:31.5974994Z   git checkout -b <new-branch-name>
2019-07-25T11:31:31.5975218Z 
2019-07-25T11:31:31.5975268Z HEAD is now at 4e5ab1258 Merge 3d5a13b7a44340be4f3754091f5152fc5e815f09 into eedf6ce4ef54bb03818ab21d714f1b9f13a6b31c
2019-07-25T11:31:31.6151316Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-25T11:31:31.6154833Z ==============================================================================
2019-07-25T11:31:31.6154893Z Task         : Bash
2019-07-25T11:31:31.6154941Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-25T11:38:52.8122510Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-07-25T11:38:54.1659808Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-07-25T11:38:55.7900677Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-07-25T11:39:07.6832335Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-07-25T11:39:11.1780708Z error[E0412]: cannot find type `UndefinedBehaviourMessage` in this scope
2019-07-25T11:39:11.1797630Z     |
2019-07-25T11:39:11.1797630Z     |
2019-07-25T11:39:11.1804023Z 326 |     UndefinedBehaviour(UndefinedBehaviourMessage<'tcx>),
2019-07-25T11:39:11.1814942Z 
2019-07-25T11:39:11.1814942Z 
2019-07-25T11:39:11.1894694Z error[E0412]: cannot find type `UnsupportedMessage` in this scope
2019-07-25T11:39:11.1904523Z     |
2019-07-25T11:39:11.1904523Z     |
2019-07-25T11:39:11.1904771Z 329 |     Unsupported(UnsupportedMessage<'tcx>),
2019-07-25T11:39:11.1910566Z 
2019-07-25T11:39:11.1910566Z 
2019-07-25T11:39:11.2003910Z error[E0412]: cannot find type `ResourceExhaustionMessage` in this scope
2019-07-25T11:39:11.2004593Z     |
2019-07-25T11:39:11.2004593Z     |
2019-07-25T11:39:11.2005376Z 334 |     ResourceExhaustion(ResourceExhaustionMessage<'tcx>),
2019-07-25T11:39:11.2005791Z 
2019-07-25T11:39:11.2005791Z 
2019-07-25T11:39:13.1952317Z error: lifetime parameter `'tcx` never used
2019-07-25T11:39:13.1952795Z     |
2019-07-25T11:39:13.1952795Z     |
2019-07-25T11:39:13.1953002Z 313 | pub enum InvalidProgramMessage<'tcx> {
2019-07-25T11:39:13.1953586Z     |                               -^^^^- help: elide the unused lifetime
2019-07-25T11:39:13.1953958Z note: lint level defined here
2019-07-25T11:39:13.1954141Z    --> src/librustc/lib.rs:32:9
2019-07-25T11:39:13.1954296Z     |
2019-07-25T11:39:13.1954296Z     |
2019-07-25T11:39:13.1954507Z 32  | #![deny(unused_lifetimes)]
2019-07-25T11:39:13.1954730Z 
2019-07-25T11:39:13.6062232Z error: aborting due to 4 previous errors
2019-07-25T11:39:13.6062480Z 
2019-07-25T11:39:13.6062753Z For more information about this error, try `rustc --explain E0412`.
2019-07-25T11:39:13.6062753Z For more information about this error, try `rustc --explain E0412`.
2019-07-25T11:39:13.7509512Z error: Could not compile `rustc`.
2019-07-25T11:39:13.7509607Z 
2019-07-25T11:39:13.7509921Z To learn more, run the command again with --verbose.
2019-07-25T11:39:13.7539306Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-25T11:39:13.7550946Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-25T11:39:13.7551004Z Build completed unsuccessfully in 0:04:36
2019-07-25T11:39:13.7551004Z Build completed unsuccessfully in 0:04:36
2019-07-25T11:39:14.9059601Z ##[error]Bash exited with code '1'.
2019-07-25T11:39:14.9094980Z ##[section]Starting: Checkout
2019-07-25T11:39:14.9097477Z ==============================================================================
2019-07-25T11:39:14.9097568Z Task         : Get sources
2019-07-25T11:39:14.9097615Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
