plain
2019-07-31T22:58:48.4035534Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-31T22:58:48.4217823Z ##[command]git config gc.auto 0
2019-07-31T22:58:48.4279666Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-31T22:58:48.4326202Z ##[command]git config --get-all http.proxy
2019-07-31T22:58:48.4451465Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63177/merge:refs/remotes/pull/63177/merge
---
2019-07-31T22:59:24.6419252Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-31T22:59:24.6419295Z 
2019-07-31T22:59:24.6419482Z   git checkout -b <new-branch-name>
2019-07-31T22:59:24.6419507Z 
2019-07-31T22:59:24.6419560Z HEAD is now at 2cc45b074 Merge 05df16d423250b3f7b47b74df9d657cdb06188d2 into 8a58268b5ad9c4a240be349a633069d48991eb0c
2019-07-31T22:59:24.6566061Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-31T22:59:24.6568783Z ==============================================================================
2019-07-31T22:59:24.6568845Z Task         : Bash
2019-07-31T22:59:24.6568884Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-31T23:04:10.9758552Z    |
2019-07-31T23:04:10.9758825Z 76 | #![feature(const_generics)]
2019-07-31T23:04:10.9759109Z    |            ^^^^^^^^^^^^^^
2019-07-31T23:04:10.9759148Z 
2019-07-31T23:04:13.7913168Z error[E0545]: incorrect 'issue'
2019-07-31T23:04:13.7913975Z     --> src/libcore/iter/traits/iterator.rs:1988:5
2019-07-31T23:04:13.7914285Z      |
2019-07-31T23:04:13.7914623Z 1988 |     #[unstable(feature = "find_result", reason = "new API", issue = "?")]
2019-07-31T23:04:13.7915012Z 
2019-07-31T23:04:13.7915012Z 
2019-07-31T23:04:17.3512358Z error[E0599]: no method named `break_value` found for type `result::Result<(), result::Result<<Self as iter::traits::iterator::Iterator>::Item, E>>` in the current scope
2019-07-31T23:04:17.3512818Z     --> src/libcore/iter/traits/iterator.rs:1999:12
2019-07-31T23:04:17.3513050Z      |
2019-07-31T23:04:17.3513342Z 1999 |         }).break_value().transpose()
2019-07-31T23:04:17.3513952Z      | 
2019-07-31T23:04:17.3514194Z     ::: src/libcore/result.rs:246:1
2019-07-31T23:04:17.3514415Z      |
2019-07-31T23:04:17.3514415Z      |
2019-07-31T23:04:17.3514670Z 246  | pub enum Result<T, E> {
2019-07-31T23:04:17.3514984Z      | --------------------- method `break_value` not found for this
2019-07-31T23:04:18.9449098Z    Compiling libc v0.2.54
2019-07-31T23:04:19.7115692Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-07-31T23:04:20.6802209Z error: aborting due to 2 previous errors
2019-07-31T23:04:20.6802311Z 
2019-07-31T23:04:20.6802311Z 
2019-07-31T23:04:20.6802612Z For more information about this error, try `rustc --explain E0599`.
2019-07-31T23:04:20.7677901Z error: Could not compile `core`.
2019-07-31T23:04:20.7679166Z warning: build failed, waiting for other jobs to finish...
2019-07-31T23:04:21.0261434Z error: build failed
2019-07-31T23:04:21.0282157Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-07-31T23:04:21.0294394Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-31T23:04:21.0294461Z Build completed unsuccessfully in 0:02:07
2019-07-31T23:04:21.0294461Z Build completed unsuccessfully in 0:02:07
2019-07-31T23:04:32.4940728Z ##[error]Bash exited with code '1'.
2019-07-31T23:04:32.4972964Z ##[section]Starting: Checkout
2019-07-31T23:04:32.4974593Z ==============================================================================
2019-07-31T23:04:32.4974659Z Task         : Get sources
2019-07-31T23:04:32.4974698Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
