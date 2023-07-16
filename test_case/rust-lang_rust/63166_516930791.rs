plain
2019-07-31T16:43:30.5944185Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-31T16:43:31.4915385Z ##[command]git config gc.auto 0
2019-07-31T16:43:31.4922059Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-31T16:43:31.4925052Z ##[command]git config --get-all http.proxy
2019-07-31T16:43:31.4928858Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63166/merge:refs/remotes/pull/63166/merge
---
2019-07-31T16:44:06.8183607Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-31T16:44:06.8184608Z 
2019-07-31T16:44:06.8185928Z   git checkout -b <new-branch-name>
2019-07-31T16:44:06.8187194Z 
2019-07-31T16:44:06.8188150Z HEAD is now at b022f4b06 Merge a0ab5a3651f5cb30efa2f82a3ee15a7df479080a into 9152fe4ea053a29469691349f4b63aa94c9aac56
2019-07-31T16:44:06.8355056Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-31T16:44:06.8358096Z ==============================================================================
2019-07-31T16:44:06.8358156Z Task         : Bash
2019-07-31T16:44:06.8358218Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-31T16:49:20.5440605Z    |
2019-07-31T16:49:20.5441565Z 76 | #![feature(const_generics)]
2019-07-31T16:49:20.5442511Z    |            ^^^^^^^^^^^^^^
2019-07-31T16:49:20.5442579Z 
2019-07-31T16:49:23.4425209Z error[E0545]: incorrect 'issue'
2019-07-31T16:49:23.4425937Z     |
2019-07-31T16:49:23.4425937Z     |
2019-07-31T16:49:23.4426273Z 837 |     #[unstable(feature = "result_copied", reason = "newly added", issue = "XXXXX")]
2019-07-31T16:49:23.4426685Z 
2019-07-31T16:49:23.4426685Z 
2019-07-31T16:49:23.4426944Z error[E0545]: incorrect 'issue'
2019-07-31T16:49:23.4427470Z     |
2019-07-31T16:49:23.4427470Z     |
2019-07-31T16:49:23.4427815Z 857 |     #[unstable(feature = "result_copied", reason = "newly added", issue = "XXXXX")]
2019-07-31T16:49:23.4428191Z 
2019-07-31T16:49:23.4428191Z 
2019-07-31T16:49:23.4428433Z error[E0545]: incorrect 'issue'
2019-07-31T16:49:23.4428929Z     |
2019-07-31T16:49:23.4428929Z     |
2019-07-31T16:49:23.4429257Z 877 |     #[unstable(feature = "result_copied", reason = "newly added", issue = "XXXXX")]
2019-07-31T16:49:23.4429828Z 
2019-07-31T16:49:23.4429828Z 
2019-07-31T16:49:23.4430114Z error[E0545]: incorrect 'issue'
2019-07-31T16:49:23.4430609Z     |
2019-07-31T16:49:23.4430609Z     |
2019-07-31T16:49:23.4431314Z 897 |     #[unstable(feature = "result_copied", reason = "newly added", issue = "XXXXX")]
2019-07-31T16:49:23.4431754Z 
2019-07-31T16:49:23.4431754Z 
2019-07-31T16:49:23.4431993Z error[E0545]: incorrect 'issue'
2019-07-31T16:49:23.4432499Z     |
2019-07-31T16:49:23.4432499Z     |
2019-07-31T16:49:23.4432985Z 917 |     #[unstable(feature = "result_cloned", reason = "newly added", issue = "XXXXX")]
2019-07-31T16:49:23.4433375Z 
2019-07-31T16:49:23.4433375Z 
2019-07-31T16:49:23.4433615Z error[E0545]: incorrect 'issue'
2019-07-31T16:49:23.4434106Z     |
2019-07-31T16:49:23.4434106Z     |
2019-07-31T16:49:23.4434433Z 937 |     #[unstable(feature = "result_cloned", reason = "newly added", issue = "XXXXX")]
2019-07-31T16:49:23.4434962Z 
2019-07-31T16:49:23.4434962Z 
2019-07-31T16:49:23.4435202Z error[E0545]: incorrect 'issue'
2019-07-31T16:49:23.4435676Z     |
2019-07-31T16:49:23.4435676Z     |
2019-07-31T16:49:23.4435988Z 957 |     #[unstable(feature = "result_cloned", reason = "newly added", issue = "XXXXX")]
2019-07-31T16:49:23.4436484Z 
2019-07-31T16:49:23.4436484Z 
2019-07-31T16:49:23.4436724Z error[E0545]: incorrect 'issue'
2019-07-31T16:49:23.4437215Z     |
2019-07-31T16:49:23.4437215Z     |
2019-07-31T16:49:23.4437549Z 977 |     #[unstable(feature = "result_cloned", reason = "newly added", issue = "XXXXX")]
2019-07-31T16:49:23.4437946Z 
2019-07-31T16:49:23.4437946Z 
2019-07-31T16:49:24.2402089Z error[E0592]: duplicate definitions with name `cloned_err`
2019-07-31T16:49:24.2404943Z     |
2019-07-31T16:49:24.2405637Z 958 | /     fn cloned_err(self) -> Result<T, E> {
2019-07-31T16:49:24.2405637Z 958 | /     fn cloned_err(self) -> Result<T, E> {
2019-07-31T16:49:24.2406290Z 959 | |         self.map_err(|e| e.clone())
2019-07-31T16:49:24.2407885Z     | |_____^ duplicate definitions for `cloned_err`
2019-07-31T16:49:24.2408436Z ...
2019-07-31T16:49:24.2409083Z 978 | /     fn cloned_err(self) -> Result<T, E> {
2019-07-31T16:49:24.2409083Z 978 | /     fn cloned_err(self) -> Result<T, E> {
2019-07-31T16:49:24.2409712Z 979 | |         self.map_err(|e| e.clone())
2019-07-31T16:49:24.2410927Z     | |_____- other definition for `cloned_err`
2019-07-31T16:49:24.2411578Z 
2019-07-31T16:49:24.2424487Z error: aborting due to 9 previous errors
2019-07-31T16:49:24.2425061Z 
2019-07-31T16:49:24.2425061Z 
2019-07-31T16:49:24.2425670Z For more information about this error, try `rustc --explain E0592`.
2019-07-31T16:49:24.3135628Z error: Could not compile `core`.
2019-07-31T16:49:24.3136736Z warning: build failed, waiting for other jobs to finish...
2019-07-31T16:49:27.1119159Z error: build failed
2019-07-31T16:49:27.1144838Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-07-31T16:49:27.1158819Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-31T16:49:27.1159152Z Build completed unsuccessfully in 0:02:17
2019-07-31T16:49:27.1159152Z Build completed unsuccessfully in 0:02:17
2019-07-31T16:49:40.7118914Z ##[error]Bash exited with code '1'.
2019-07-31T16:49:40.7153345Z ##[section]Starting: Checkout
2019-07-31T16:49:40.7155417Z ==============================================================================
2019-07-31T16:49:40.7155471Z Task         : Get sources
2019-07-31T16:49:40.7155535Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
