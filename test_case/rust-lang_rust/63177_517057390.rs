plain
2019-07-31T23:11:32.5430032Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-31T23:11:32.5620022Z ##[command]git config gc.auto 0
2019-07-31T23:11:32.6012608Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-31T23:11:32.6061055Z ##[command]git config --get-all http.proxy
2019-07-31T23:11:32.6186597Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63177/merge:refs/remotes/pull/63177/merge
---
2019-07-31T23:12:06.4441344Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-31T23:12:06.4441378Z 
2019-07-31T23:12:06.4441604Z   git checkout -b <new-branch-name>
2019-07-31T23:12:06.4441634Z 
2019-07-31T23:12:06.4441704Z HEAD is now at c40986eae Merge 80cab9dc63ab786287444e485a5f0ee9ccd4218a into 8a58268b5ad9c4a240be349a633069d48991eb0c
2019-07-31T23:12:06.4605199Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-31T23:12:06.4607997Z ==============================================================================
2019-07-31T23:12:06.4608085Z Task         : Bash
2019-07-31T23:12:06.4608127Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-31T23:17:07.3342712Z    |
2019-07-31T23:17:07.3342978Z 76 | #![feature(const_generics)]
2019-07-31T23:17:07.3343742Z    |            ^^^^^^^^^^^^^^
2019-07-31T23:17:07.3343810Z 
2019-07-31T23:17:10.3828621Z error[E0545]: incorrect 'issue'
2019-07-31T23:17:10.3829099Z     --> src/libcore/iter/traits/iterator.rs:1988:5
2019-07-31T23:17:10.3829376Z      |
2019-07-31T23:17:10.3829719Z 1988 |     #[unstable(feature = "find_result", reason = "new API", issue = "?")]
2019-07-31T23:17:10.3830138Z 
2019-07-31T23:17:10.3830138Z 
2019-07-31T23:17:14.1981160Z error[E0599]: no method named `break_value` found for type `result::Result<(), result::Result<<Self as iter::traits::iterator::Iterator>::Item, E>>` in the current scope
2019-07-31T23:17:14.1981600Z     --> src/libcore/iter/traits/iterator.rs:1999:12
2019-07-31T23:17:14.1981972Z      |
2019-07-31T23:17:14.1982262Z 1999 |         }).break_value().transpose()
2019-07-31T23:17:14.1982684Z      | 
2019-07-31T23:17:14.1982919Z     ::: src/libcore/result.rs:246:1
2019-07-31T23:17:14.1983106Z      |
2019-07-31T23:17:14.1983106Z      |
2019-07-31T23:17:14.1983329Z 246  | pub enum Result<T, E> {
2019-07-31T23:17:14.1984378Z      | --------------------- method `break_value` not found for this
2019-07-31T23:17:15.8630006Z    Compiling libc v0.2.54
2019-07-31T23:17:16.6813516Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-07-31T23:17:17.7756351Z error: aborting due to 2 previous errors
2019-07-31T23:17:17.7756734Z 
2019-07-31T23:17:17.7756734Z 
2019-07-31T23:17:17.7757154Z For more information about this error, try `rustc --explain E0599`.
2019-07-31T23:17:17.8701626Z error: Could not compile `core`.
2019-07-31T23:17:17.8703709Z warning: build failed, waiting for other jobs to finish...
2019-07-31T23:17:18.1220153Z error: build failed
2019-07-31T23:17:18.1241861Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-07-31T23:17:18.1255284Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-31T23:17:18.1255520Z Build completed unsuccessfully in 0:02:14
2019-07-31T23:17:18.1255520Z Build completed unsuccessfully in 0:02:14
2019-07-31T23:17:31.4296441Z ##[error]Bash exited with code '1'.
2019-07-31T23:17:31.4331656Z ##[section]Starting: Checkout
2019-07-31T23:17:31.4333197Z ==============================================================================
2019-07-31T23:17:31.4333270Z Task         : Get sources
2019-07-31T23:17:31.4333317Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
