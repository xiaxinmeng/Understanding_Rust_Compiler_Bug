plain
2019-08-10T14:48:02.6219688Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-10T14:48:02.6401805Z ##[command]git config gc.auto 0
2019-08-10T14:48:02.6455292Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-10T14:48:02.6501621Z ##[command]git config --get-all http.proxy
2019-08-10T14:48:02.6636843Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63441/merge:refs/remotes/pull/63441/merge
---
2019-08-10T14:48:37.3673899Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-10T14:48:37.3673975Z 
2019-08-10T14:48:37.3674190Z   git checkout -b <new-branch-name>
2019-08-10T14:48:37.3674219Z 
2019-08-10T14:48:37.3674268Z HEAD is now at cb9749599 Merge d809d6ee899154a0ba99e6159ba9d54330ebf0ac into 6f70adcb18e5dc8df0672898a8404fd05a9c32cb
2019-08-10T14:48:37.3830274Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-10T14:48:37.3833576Z ==============================================================================
2019-08-10T14:48:37.3833634Z Task         : Bash
2019-08-10T14:48:37.3833680Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-10T14:56:36.2703047Z     Checking rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2019-08-10T14:56:38.7175504Z     Checking rustc_plugin v0.0.0 (/checkout/src/librustc_plugin)
2019-08-10T14:56:38.9966123Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2019-08-10T14:56:39.5336499Z     Checking rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2019-08-10T14:56:40.4835266Z error[E0277]: `rustc::middle::cstore::NativeLibrary` doesn't implement `std::fmt::Debug`
2019-08-10T14:56:40.4842352Z    --> src/librustc_codegen_ssa/lib.rs:138:5
2019-08-10T14:56:40.4848329Z     |
2019-08-10T14:56:40.4854679Z 138 |     pub native_libraries: FxHashMap<CrateNum, Lrc<Vec<NativeLibrary>>>,
2019-08-10T14:56:40.4860704Z     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `rustc::middle::cstore::NativeLibrary` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
2019-08-10T14:56:40.4867438Z     |
2019-08-10T14:56:40.4889443Z     = help: the trait `std::fmt::Debug` is not implemented for `rustc::middle::cstore::NativeLibrary`
2019-08-10T14:56:40.4890038Z     = note: required because of the requirements on the impl of `std::fmt::Debug` for `std::vec::Vec<rustc::middle::cstore::NativeLibrary>`
2019-08-10T14:56:40.4890461Z     = note: required because of the requirements on the impl of `std::fmt::Debug` for `std::sync::Arc<std::vec::Vec<rustc::middle::cstore::NativeLibrary>>`
2019-08-10T14:56:40.4890917Z     = note: required because of the requirements on the impl of `std::fmt::Debug` for `std::collections::HashMap<rustc::hir::def_id::CrateNum, std::sync::Arc<std::vec::Vec<rustc::middle::cstore::NativeLibrary>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
2019-08-10T14:56:40.4900308Z     = note: required because of the requirements on the impl of `std::fmt::Debug` for `&std::collections::HashMap<rustc::hir::def_id::CrateNum, std::sync::Arc<std::vec::Vec<rustc::middle::cstore::NativeLibrary>>, std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
2019-08-10T14:56:40.4900624Z     = note: required for the cast to the object type `dyn std::fmt::Debug`
2019-08-10T14:56:40.4900680Z 
2019-08-10T14:56:40.4900989Z error[E0277]: `rustc::middle::cstore::NativeLibrary` doesn't implement `std::fmt::Debug`
2019-08-10T14:56:40.4954421Z    --> src/librustc_codegen_ssa/lib.rs:140:5
2019-08-10T14:56:40.4954721Z     |
2019-08-10T14:56:40.4955018Z 140 |     pub used_libraries: Lrc<Vec<NativeLibrary>>,
2019-08-10T14:56:40.4955610Z     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `rustc::middle::cstore::NativeLibrary` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
2019-08-10T14:56:40.4956003Z     |
2019-08-10T14:56:40.4956291Z     = help: the trait `std::fmt::Debug` is not implemented for `rustc::middle::cstore::NativeLibrary`
2019-08-10T14:56:40.4956624Z     = note: required because of the requirements on the impl of `std::fmt::Debug` for `std::vec::Vec<rustc::middle::cstore::NativeLibrary>`
2019-08-10T14:56:40.4956949Z     = note: required because of the requirements on the impl of `std::fmt::Debug` for `std::sync::Arc<std::vec::Vec<rustc::middle::cstore::NativeLibrary>>`
2019-08-10T14:56:40.4957293Z     = note: required because of the requirements on the impl of `std::fmt::Debug` for `&std::sync::Arc<std::vec::Vec<rustc::middle::cstore::NativeLibrary>>`
2019-08-10T14:56:40.4957565Z     = note: required for the cast to the object type `dyn std::fmt::Debug`
2019-08-10T14:56:42.1663699Z error: aborting due to 2 previous errors
2019-08-10T14:56:42.1664826Z 
2019-08-10T14:56:42.1670447Z For more information about this error, try `rustc --explain E0277`.
2019-08-10T14:56:42.1920010Z error: Could not compile `rustc_codegen_ssa`.
2019-08-10T14:56:42.1920010Z error: Could not compile `rustc_codegen_ssa`.
2019-08-10T14:56:42.1936714Z warning: build failed, waiting for other jobs to finish...
2019-08-10T14:56:45.1082614Z error: build failed
2019-08-10T14:56:45.1111637Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-08-10T14:56:45.1112461Z expected success, got: exit code: 101
2019-08-10T14:56:45.1127318Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-10T14:56:45.1127877Z Build completed unsuccessfully in 0:05:29
2019-08-10T14:56:46.3408636Z ##[error]Bash exited with code '1'.
2019-08-10T14:56:46.3438548Z ##[section]Starting: Checkout
2019-08-10T14:56:46.3440022Z ==============================================================================
2019-08-10T14:56:46.3440070Z Task         : Get sources
2019-08-10T14:56:46.3440112Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
