plain
2019-08-10T10:40:05.9022698Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-10T10:40:05.9179539Z ##[command]git config gc.auto 0
2019-08-10T10:40:05.9250099Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-10T10:40:05.9292008Z ##[command]git config --get-all http.proxy
2019-08-10T10:40:05.9421802Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63343/merge:refs/remotes/pull/63343/merge
---
2019-08-10T10:40:39.3330293Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-10T10:40:39.3330328Z 
2019-08-10T10:40:39.3330562Z   git checkout -b <new-branch-name>
2019-08-10T10:40:39.3330592Z 
2019-08-10T10:40:39.3330666Z HEAD is now at e93594ab1 Merge 20326f44e0668618fdefbfef5fc8ff867506522f into d19a359444295bab01de7ff44a9d72302e573bc9
2019-08-10T10:40:39.3476583Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-10T10:40:39.3478831Z ==============================================================================
2019-08-10T10:40:39.3478876Z Task         : Bash
2019-08-10T10:40:39.3478909Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-10T11:09:41.8291561Z Assembling stage1 compiler (x86_64-unknown-linux-gnu)
2019-08-10T11:09:41.8317138Z Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-08-10T11:09:42.2533491Z    Compiling cc v1.0.35
2019-08-10T11:09:42.2534721Z    Compiling core v0.0.0 (/checkout/src/libcore)
2019-08-10T11:09:46.4737766Z error: use of deprecated item 'intrinsics::init': superseded by MaybeUninit, removal planned
2019-08-10T11:09:46.4739132Z    --> src/libcore/mem/mod.rs:458:5
2019-08-10T11:09:46.4741180Z 458 |     intrinsics::init()
2019-08-10T11:09:46.4742146Z     |     ^^^^^^^^^^^^^^^^
2019-08-10T11:09:46.4744171Z     |
2019-08-10T11:09:46.4745086Z     = note: `-D deprecated` implied by `-D warnings`
2019-08-10T11:09:46.4745086Z     = note: `-D deprecated` implied by `-D warnings`
2019-08-10T11:09:46.4745568Z 
2019-08-10T11:09:46.4746294Z error: use of deprecated item 'intrinsics::uninit': superseded by MaybeUninit, removal planned
2019-08-10T11:09:46.4747206Z    --> src/libcore/mem/mod.rs:485:5
2019-08-10T11:09:46.4748495Z 485 |     intrinsics::uninit()
2019-08-10T11:09:46.4749113Z     |     ^^^^^^^^^^^^^^^^^^
2019-08-10T11:09:46.4749530Z 
2019-08-10T11:09:51.0750443Z    Compiling libc v0.2.60
---
2019-08-10T11:09:55.9644658Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-08-10T11:09:55.9645040Z expected success, got: exit code: 101
2019-08-10T11:09:55.9656266Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-10T11:09:55.9656571Z Build completed unsuccessfully in 0:23:12
2019-08-10T11:09:56.9133846Z ##[error]Bash exited with code '1'.
2019-08-10T11:09:56.9178546Z ##[section]Starting: Checkout
2019-08-10T11:09:56.9180260Z ==============================================================================
2019-08-10T11:09:56.9180325Z Task         : Get sources
2019-08-10T11:09:56.9180364Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
