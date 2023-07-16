plain
2019-12-23T13:16:37.6123473Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-23T13:16:37.6135699Z ##[command]git config gc.auto 0
2019-12-23T13:16:37.6139396Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-23T13:16:37.6143309Z ##[command]git config --get-all http.proxy
2019-12-23T13:16:37.6146963Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67398/merge:refs/remotes/pull/67398/merge
---
2019-12-23T13:22:45.9805256Z     Checking rustc_apfloat v0.0.0 (/checkout/src/librustc_apfloat)
2019-12-23T13:22:46.1989660Z    Compiling quote v1.0.2
2019-12-23T13:22:46.7664173Z     Checking flate2 v1.0.12
2019-12-23T13:22:47.3808908Z     Checking rand_core v0.5.1
2019-12-23T13:22:47.6841254Z     Checking rustc_jobserver v0.0.0 (/checkout/src/librustc_jobserver)
2019-12-23T13:22:48.3889485Z     Checking serde_json v1.0.40
2019-12-23T13:22:48.5144692Z     Checking polonius-engine v0.11.0
2019-12-23T13:22:49.8632322Z     Checking chalk-engine v0.9.0
2019-12-23T13:22:51.8468763Z     Checking backtrace v0.3.40
---
2019-12-23T13:23:52.5340575Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-12-23T13:23:52.6377931Z error[E0433]: failed to resolve: could not find `static_assert_size` in `rustc_data_structures`
2019-12-23T13:23:52.6378779Z   --> src/librustc_errors/lib.rs:47:24
2019-12-23T13:23:52.6379282Z    |
2019-12-23T13:23:52.6379612Z 47 | rustc_data_structures::static_assert_size!(PResult<'_, bool>, 16);
2019-12-23T13:23:52.6380076Z 
2019-12-23T13:23:53.2422071Z error: aborting due to previous error
2019-12-23T13:23:53.2423013Z 
2019-12-23T13:23:53.2423382Z For more information about this error, try `rustc --explain E0433`.
2019-12-23T13:23:53.2423382Z For more information about this error, try `rustc --explain E0433`.
2019-12-23T13:23:53.2482712Z error: could not compile `rustc_errors`.
2019-12-23T13:23:53.2505765Z warning: build failed, waiting for other jobs to finish...
2019-12-23T13:23:55.2913485Z error: build failed
2019-12-23T13:23:55.2940928Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-23T13:23:55.2949266Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-12-23T13:23:55.2949649Z Build completed unsuccessfully in 0:03:56
2019-12-23T13:23:55.3001742Z == clock drift check ==
2019-12-23T13:23:55.3015936Z   local time: Mon Dec 23 13:23:55 UTC 2019
2019-12-23T13:23:55.3015936Z   local time: Mon Dec 23 13:23:55 UTC 2019
2019-12-23T13:23:55.3552748Z   network time: Mon, 23 Dec 2019 13:23:55 GMT
2019-12-23T13:23:55.3556882Z == end clock drift check ==
2019-12-23T13:23:56.7048501Z 
2019-12-23T13:23:56.7157458Z ##[error]Bash exited with code '1'.
2019-12-23T13:23:56.7184476Z ##[section]Starting: Checkout
2019-12-23T13:23:56.7185924Z ==============================================================================
2019-12-23T13:23:56.7185976Z Task         : Get sources
2019-12-23T13:23:56.7186037Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
