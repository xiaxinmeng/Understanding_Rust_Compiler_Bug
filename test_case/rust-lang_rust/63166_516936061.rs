plain
2019-07-31T16:56:58.1201497Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-31T16:56:58.1434030Z ##[command]git config gc.auto 0
2019-07-31T16:56:58.1490305Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-31T16:56:58.1547275Z ##[command]git config --get-all http.proxy
2019-07-31T16:56:58.1701718Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63166/merge:refs/remotes/pull/63166/merge
---
2019-07-31T16:57:33.1527710Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-31T16:57:33.1527747Z 
2019-07-31T16:57:33.1527978Z   git checkout -b <new-branch-name>
2019-07-31T16:57:33.1528011Z 
2019-07-31T16:57:33.1528275Z HEAD is now at 6585cec46 Merge c784720f3a2d0b66142da3c9a9fd6039f5d3a036 into 9152fe4ea053a29469691349f4b63aa94c9aac56
2019-07-31T16:57:33.1678781Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-31T16:57:33.1681901Z ==============================================================================
2019-07-31T16:57:33.1681987Z Task         : Bash
2019-07-31T16:57:33.1682041Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-31T17:02:56.2852452Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-07-31T17:02:57.9253723Z    Compiling autocfg v0.1.4
2019-07-31T17:02:59.5954321Z    Compiling compiler_builtins v0.1.18
2019-07-31T17:03:02.6057482Z    Compiling cmake v0.1.38
2019-07-31T17:03:02.6987701Z error: method is never used: `copied`
2019-07-31T17:03:02.7003756Z     |
2019-07-31T17:03:02.7003756Z     |
2019-07-31T17:03:02.7010798Z 838 |     fn copied(self) -> Result<T, E> {
2019-07-31T17:03:02.7025110Z     |
2019-07-31T17:03:02.7042008Z     = note: `-D dead-code` implied by `-D warnings`
2019-07-31T17:03:02.7042311Z 
2019-07-31T17:03:02.7042311Z 
2019-07-31T17:03:02.7044412Z error: method is never used: `copied`
2019-07-31T17:03:02.7045102Z     |
2019-07-31T17:03:02.7045102Z     |
2019-07-31T17:03:02.7045394Z 858 |     fn copied(self) -> Result<T, E> {
2019-07-31T17:03:02.7045746Z 
2019-07-31T17:03:02.7045746Z 
2019-07-31T17:03:02.7045999Z error: method is never used: `copied_err`
2019-07-31T17:03:02.7046529Z     |
2019-07-31T17:03:02.7046529Z     |
2019-07-31T17:03:02.7046825Z 878 |     fn copied_err(self) -> Result<T, E> {
2019-07-31T17:03:02.7047188Z 
2019-07-31T17:03:02.7047188Z 
2019-07-31T17:03:02.7047444Z error: method is never used: `copied_err`
2019-07-31T17:03:02.7047949Z     |
2019-07-31T17:03:02.7047949Z     |
2019-07-31T17:03:02.7048239Z 898 |     fn copied_err(self) -> Result<T, E> {
2019-07-31T17:03:02.7048582Z 
2019-07-31T17:03:02.7048582Z 
2019-07-31T17:03:02.7048832Z error: method is never used: `cloned`
2019-07-31T17:03:02.7049349Z     |
2019-07-31T17:03:02.7049648Z 918 |     fn cloned(self) -> Result<T, E> {
2019-07-31T17:03:02.7049935Z     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-07-31T17:03:02.7049989Z 
2019-07-31T17:03:02.7049989Z 
2019-07-31T17:03:02.7050614Z error: method is never used: `cloned`
2019-07-31T17:03:02.7051130Z     |
2019-07-31T17:03:02.7051450Z 938 |     fn cloned(self) -> Result<T, E> {
2019-07-31T17:03:02.7051993Z     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-07-31T17:03:02.7052042Z 
2019-07-31T17:03:02.7052042Z 
2019-07-31T17:03:02.7052541Z error: method is never used: `cloned_err`
2019-07-31T17:03:02.7053092Z     |
2019-07-31T17:03:02.7053396Z 958 |     fn cloned_err(self) -> Result<T, E> {
2019-07-31T17:03:02.7053690Z     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-07-31T17:03:02.7053728Z 
2019-07-31T17:03:02.7053728Z 
2019-07-31T17:03:02.7053994Z error: method is never used: `cloned_err`
2019-07-31T17:03:02.7054475Z     |
2019-07-31T17:03:02.7054777Z 978 |     fn cloned_err(self) -> Result<T, E> {
2019-07-31T17:03:02.7055079Z     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-07-31T17:03:02.7055117Z 
2019-07-31T17:03:02.7055117Z 
2019-07-31T17:03:03.2712326Z error: aborting due to 8 previous errors
2019-07-31T17:03:03.2717586Z 
2019-07-31T17:03:03.4086324Z error: Could not compile `core`.
2019-07-31T17:03:03.4086755Z warning: build failed, waiting for other jobs to finish...
2019-07-31T17:03:04.5659936Z error: build failed
2019-07-31T17:03:04.5680745Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-07-31T17:03:04.5695200Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-31T17:03:04.5695341Z Build completed unsuccessfully in 0:02:21
2019-07-31T17:03:04.5695341Z Build completed unsuccessfully in 0:02:21
2019-07-31T17:03:17.1219263Z ##[error]Bash exited with code '1'.
2019-07-31T17:03:17.1253545Z ##[section]Starting: Checkout
2019-07-31T17:03:17.1255227Z ==============================================================================
2019-07-31T17:03:17.1255282Z Task         : Get sources
2019-07-31T17:03:17.1255329Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
