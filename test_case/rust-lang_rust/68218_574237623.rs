plain
2020-01-14T15:36:07.1850361Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-14T15:36:07.1860661Z ##[command]git config gc.auto 0
2020-01-14T15:36:07.1863368Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-14T15:36:07.1866321Z ##[command]git config --get-all http.proxy
2020-01-14T15:36:07.1870252Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68218/merge:refs/remotes/pull/68218/merge
---
2020-01-14T15:42:09.6149346Z     Checking measureme v0.7.1
2020-01-14T15:42:09.9157005Z    Compiling synstructure v0.12.1
2020-01-14T15:42:41.7284499Z     Checking rustc-rayon v0.3.0
2020-01-14T15:42:44.2654577Z     Checking rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2020-01-14T15:42:45.1230228Z error[E0599]: no method named `wait_until` found for type `std::sync::Condvar` in the current scope
2020-01-14T15:42:45.1230690Z   --> src/librustc_data_structures/sync/future.rs:58:18
2020-01-14T15:42:45.1230933Z    |
2020-01-14T15:42:45.1231267Z 58 |                 .wait_until(self.data.result.lock().unwrap(), |result| result.is_some())
2020-01-14T15:42:45.1232045Z    |                  ^^^^^^^^^^ method not found in `std::sync::Condvar`
2020-01-14T15:42:45.2495995Z error: aborting due to previous error
2020-01-14T15:42:45.2500035Z 
2020-01-14T15:42:45.2509545Z For more information about this error, try `rustc --explain E0599`.
2020-01-14T15:42:45.2568972Z error: could not compile `rustc_data_structures`.
2020-01-14T15:42:45.2568972Z error: could not compile `rustc_data_structures`.
2020-01-14T15:42:45.2601735Z warning: build failed, waiting for other jobs to finish...
2020-01-14T15:42:53.4135584Z error: build failed
2020-01-14T15:42:53.4141443Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-14T15:42:53.4190999Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-01-14T15:42:53.4191096Z Build completed unsuccessfully in 0:04:06
2020-01-14T15:42:53.4195424Z == clock drift check ==
2020-01-14T15:42:53.4207033Z   local time: Tue Jan 14 15:42:53 UTC 2020
2020-01-14T15:42:53.4207033Z   local time: Tue Jan 14 15:42:53 UTC 2020
2020-01-14T15:42:53.5555750Z   network time: Tue, 14 Jan 2020 15:42:53 GMT
2020-01-14T15:42:53.5558988Z == end clock drift check ==
2020-01-14T15:42:54.0836040Z 
2020-01-14T15:42:54.0936353Z ##[error]Bash exited with code '1'.
2020-01-14T15:42:54.0962452Z ##[section]Starting: Checkout
2020-01-14T15:42:54.0964148Z ==============================================================================
2020-01-14T15:42:54.0964301Z Task         : Get sources
2020-01-14T15:42:54.0964348Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
