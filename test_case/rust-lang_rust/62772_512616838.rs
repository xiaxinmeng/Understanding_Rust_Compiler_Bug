plain
2019-07-17T23:36:12.3598657Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-17T23:36:12.3791362Z ##[command]git config gc.auto 0
2019-07-17T23:36:12.3863584Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-17T23:36:12.3911570Z ##[command]git config --get-all http.proxy
2019-07-17T23:36:12.4049729Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62772/merge:refs/remotes/pull/62772/merge
---
2019-07-17T23:36:46.6332465Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-17T23:36:46.6332868Z 
2019-07-17T23:36:46.6333215Z   git checkout -b <new-branch-name>
2019-07-17T23:36:46.6333429Z 
2019-07-17T23:36:46.6333619Z HEAD is now at 918f46cb8 Merge dd24ac173c6fc84cd1318dd07c9d75bc911f15b4 into bc2e84ca0939b73fcf1768209044432f6a15c2e5
2019-07-17T23:36:46.6463579Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-17T23:36:46.6466895Z ==============================================================================
2019-07-17T23:36:46.6466943Z Task         : Bash
2019-07-17T23:36:46.6466981Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-18T00:10:41.5710978Z    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-07-18T00:17:27.5756265Z    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
2019-07-18T00:18:36.1156666Z    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-07-18T00:19:57.0039373Z    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-07-18T00:20:04.6795493Z error: usage of `ty::TyKind::<kind>`
2019-07-18T00:20:04.6796638Z    --> src/librustc_typeck/check/method/suggest.rs:689:17
2019-07-18T00:20:04.6797080Z     |
2019-07-18T00:20:04.6797714Z 689 |                 ty::TyKind::Param(param) => Some(param),
2019-07-18T00:20:04.6798131Z     |                 ^^^^^^^^^^ help: try using ty::<kind> directly: `ty`
2019-07-18T00:20:04.6798490Z     |
2019-07-18T00:20:04.6799333Z     = note: `-D rustc::usage-of-ty-tykind` implied by `-D rustc::internal`
2019-07-18T00:20:04.6799554Z 
2019-07-18T00:20:04.6799866Z error: usage of `ty::TyKind::<kind>`
2019-07-18T00:20:04.6800233Z    --> src/librustc_typeck/check/method/suggest.rs:690:17
2019-07-18T00:20:04.6800552Z     |
2019-07-18T00:20:04.6800909Z 690 |                 ty::TyKind::Ref(_, ty, _) => match ty.sty {
2019-07-18T00:20:04.6801340Z     |                 ^^^^^^^^^^ help: try using ty::<kind> directly: `ty`
2019-07-18T00:20:04.6801565Z 
2019-07-18T00:20:04.6801878Z error: usage of `ty::TyKind::<kind>`
2019-07-18T00:20:04.6802225Z    --> src/librustc_typeck/check/method/suggest.rs:691:21
2019-07-18T00:20:04.6802523Z     |
2019-07-18T00:20:04.6803154Z 691 |                     ty::TyKind::Param(param) => Some(param),
2019-07-18T00:20:04.6803594Z     |                     ^^^^^^^^^^ help: try using ty::<kind> directly: `ty`
2019-07-18T00:20:04.8161629Z error: aborting due to 3 previous errors
2019-07-18T00:20:04.8162394Z 
2019-07-18T00:20:04.8997900Z error: Could not compile `rustc_typeck`.
2019-07-18T00:20:04.8998980Z warning: build failed, waiting for other jobs to finish...
2019-07-18T00:20:04.8998980Z warning: build failed, waiting for other jobs to finish...
2019-07-18T00:23:05.5923250Z error: build failed
2019-07-18T00:23:05.5944924Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-18T00:23:05.5945067Z expected success, got: exit code: 101
2019-07-18T00:23:05.5956811Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-18T00:23:05.5957005Z Build completed unsuccessfully in 0:40:12
2019-07-18T00:23:07.3077199Z ##[error]Bash exited with code '1'.
2019-07-18T00:23:07.3138603Z ##[section]Starting: Checkout
2019-07-18T00:23:07.3140042Z ==============================================================================
2019-07-18T00:23:07.3140088Z Task         : Get sources
2019-07-18T00:23:07.3140125Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
