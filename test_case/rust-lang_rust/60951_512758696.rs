plain
2019-07-18T10:12:13.0087986Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-18T10:12:13.0269541Z ##[command]git config gc.auto 0
2019-07-18T10:12:13.0335920Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-18T10:12:13.0387017Z ##[command]git config --get-all http.proxy
2019-07-18T10:12:13.0532468Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/60951/merge:refs/remotes/pull/60951/merge
---
2019-07-18T10:12:48.3363425Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-18T10:12:48.3363454Z 
2019-07-18T10:12:48.3363682Z   git checkout -b <new-branch-name>
2019-07-18T10:12:48.3363710Z 
2019-07-18T10:12:48.3363757Z HEAD is now at 28590e917 Merge 589f6a7dc66d88739485a1f9044ab29c166420a0 into 2c3b05d90d4dee879a0d9f33e721c930b087d7d1
2019-07-18T10:12:48.3498603Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-18T10:12:48.3501056Z ==============================================================================
2019-07-18T10:12:48.3501100Z Task         : Bash
2019-07-18T10:12:48.3501153Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-18T10:20:06.1123210Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-07-18T10:20:07.3887164Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-07-18T10:20:08.4994844Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-07-18T10:20:19.9285186Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-07-18T10:20:23.2089775Z error[E0412]: cannot find type `FnSig` in this scope
2019-07-18T10:20:23.2090156Z    --> src/librustc/mir/interpret/error.rs:250:31
2019-07-18T10:20:23.2090477Z     |
2019-07-18T10:20:23.2097115Z 250 |     FunctionPointerTyMismatch(FnSig<'tcx>, FnSig<'tcx>),
2019-07-18T10:20:23.2098558Z help: possible candidate is found in another module, you can import it into scope
2019-07-18T10:20:23.2099022Z     |
2019-07-18T10:20:23.2099022Z     |
2019-07-18T10:20:23.2099533Z 1   | use crate::ty::sty::FnSig;
2019-07-18T10:20:23.2100172Z 
2019-07-18T10:20:23.2100172Z 
2019-07-18T10:20:23.2187554Z error[E0412]: cannot find type `FnSig` in this scope
2019-07-18T10:20:23.2188123Z    --> src/librustc/mir/interpret/error.rs:250:44
2019-07-18T10:20:23.2188445Z     |
2019-07-18T10:20:23.2188760Z 250 |     FunctionPointerTyMismatch(FnSig<'tcx>, FnSig<'tcx>),
2019-07-18T10:20:23.2189449Z help: possible candidate is found in another module, you can import it into scope
2019-07-18T10:20:23.2189709Z     |
2019-07-18T10:20:23.2189709Z     |
2019-07-18T10:20:23.2190042Z 1   | use crate::ty::sty::FnSig;
2019-07-18T10:20:23.2190317Z 
2019-07-18T10:20:23.2190317Z 
2019-07-18T10:20:23.2278745Z error[E0412]: cannot find type `AccessKind` in this scope
2019-07-18T10:20:23.2279103Z    --> src/librustc/mir/interpret/error.rs:291:17
2019-07-18T10:20:23.2279333Z     |
2019-07-18T10:20:23.2279625Z 291 |         access: AccessKind,
2019-07-18T10:20:23.2279999Z 
2019-07-18T10:20:23.2279999Z 
2019-07-18T10:20:23.2375128Z error[E0412]: cannot find type `Lock` in this scope
2019-07-18T10:20:23.2375424Z    --> src/librustc/mir/interpret/error.rs:292:15
2019-07-18T10:20:23.2376218Z 292 |         lock: Lock,
2019-07-18T10:20:23.2402097Z     |               ^^^^ not found in this scope
2019-07-18T10:20:23.2402097Z     |               ^^^^ not found in this scope
2019-07-18T10:20:23.2402405Z help: possible candidates are found in other modules, you can import them into scope
2019-07-18T10:20:23.2403079Z 1   | use rustc_data_structures::flock::Lock;
2019-07-18T10:20:23.2403281Z     |
2019-07-18T10:20:23.2403541Z 1   | use rustc_data_structures::sync::Lock;
2019-07-18T10:20:23.2403750Z     |
2019-07-18T10:20:23.2403750Z     |
2019-07-18T10:20:23.2403782Z 
2019-07-18T10:20:23.2474614Z error[E0412]: cannot find type `AccessKind` in this scope
2019-07-18T10:20:23.2475124Z    --> src/librustc/mir/interpret/error.rs:297:15
2019-07-18T10:20:23.2475330Z     |
2019-07-18T10:20:23.2475595Z 297 |         kind: AccessKind,
2019-07-18T10:20:23.2475957Z 
2019-07-18T10:20:23.2475957Z 
2019-07-18T10:20:23.2568563Z error[E0412]: cannot find type `Lock` in this scope
2019-07-18T10:20:23.2568927Z    --> src/librustc/mir/interpret/error.rs:298:15
2019-07-18T10:20:23.2569520Z 298 |         lock: Lock,
2019-07-18T10:20:23.2569902Z     |               ^^^^ not found in this scope
2019-07-18T10:20:23.2569902Z     |               ^^^^ not found in this scope
2019-07-18T10:20:23.2570267Z help: possible candidates are found in other modules, you can import them into scope
2019-07-18T10:20:23.2571375Z 1   | use rustc_data_structures::flock::Lock;
2019-07-18T10:20:23.2571604Z     |
2019-07-18T10:20:23.2571856Z 1   | use rustc_data_structures::sync::Lock;
2019-07-18T10:20:23.2572061Z     |
2019-07-18T10:20:23.2572061Z     |
2019-07-18T10:20:23.2572108Z 
2019-07-18T10:20:23.2665428Z error[E0412]: cannot find type `Lock` in this scope
2019-07-18T10:20:23.2665948Z    --> src/librustc/mir/interpret/error.rs:304:15
2019-07-18T10:20:23.2666496Z 304 |         lock: Lock,
2019-07-18T10:20:23.2666795Z     |               ^^^^ not found in this scope
2019-07-18T10:20:23.2666795Z     |               ^^^^ not found in this scope
2019-07-18T10:20:23.2667117Z help: possible candidates are found in other modules, you can import them into scope
2019-07-18T10:20:23.2668831Z 1   | use rustc_data_structures::flock::Lock;
2019-07-18T10:20:23.2669108Z     |
2019-07-18T10:20:23.2669415Z 1   | use rustc_data_structures::sync::Lock;
2019-07-18T10:20:23.2669660Z     |
2019-07-18T10:20:23.2669660Z     |
2019-07-18T10:20:23.2669717Z 
2019-07-18T10:20:23.2773438Z error[E0412]: cannot find type `Lock` in this scope
2019-07-18T10:20:23.2773902Z    --> src/librustc/mir/interpret/error.rs:308:15
2019-07-18T10:20:23.2774354Z 308 |         lock: Lock,
2019-07-18T10:20:23.2774802Z     |               ^^^^ not found in this scope
2019-07-18T10:20:23.2774802Z     |               ^^^^ not found in this scope
2019-07-18T10:20:23.2775075Z help: possible candidates are found in other modules, you can import them into scope
2019-07-18T10:20:23.2775503Z 1   | use rustc_data_structures::flock::Lock;
2019-07-18T10:20:23.2775709Z     |
2019-07-18T10:20:23.2775944Z 1   | use rustc_data_structures::sync::Lock;
2019-07-18T10:20:23.2776139Z     |
2019-07-18T10:20:23.2776139Z     |
2019-07-18T10:20:23.2776184Z 
2019-07-18T10:20:24.0419164Z error: unused import: `syntax::symbol::Symbol`
2019-07-18T10:20:24.0419567Z   --> src/librustc/mir/interpret/error.rs:19:5
2019-07-18T10:20:24.0419837Z    |
2019-07-18T10:20:24.0420109Z 19 | use syntax::symbol::Symbol;
2019-07-18T10:20:24.0420379Z    |     ^^^^^^^^^^^^^^^^^^^^^^
2019-07-18T10:20:24.0420613Z    |
2019-07-18T10:20:24.0421039Z    = note: `-D unused-imports` implied by `-D warnings`
2019-07-18T10:20:24.0421079Z 
2019-07-18T10:20:25.1634833Z error: lifetime parameter `'tcx` never used
2019-07-18T10:20:25.1635405Z    --> src/librustc/mir/interpret/error.rs:344:25
2019-07-18T10:20:25.1635574Z     |
2019-07-18T10:20:25.1635779Z 344 | pub enum EvalErrorPanic<'tcx, O> {
2019-07-18T10:20:25.1636251Z     |                         ^^^^--
2019-07-18T10:20:25.1636516Z     |                         |
2019-07-18T10:20:25.1636765Z     |                         help: elide the unused lifetime
2019-07-18T10:20:25.1637104Z note: lint level defined here
2019-07-18T10:20:25.1637285Z    --> src/librustc/lib.rs:32:9
2019-07-18T10:20:25.1637773Z     |
2019-07-18T10:20:25.1637773Z     |
2019-07-18T10:20:25.1638042Z 32  | #![deny(unused_lifetimes)]
2019-07-18T10:20:25.1638365Z 
2019-07-18T10:20:25.5852749Z error: aborting due to 10 previous errors
2019-07-18T10:20:25.5852840Z 
2019-07-18T10:20:25.5853084Z For more information about this error, try `rustc --explain E0412`.
2019-07-18T10:20:25.5853084Z For more information about this error, try `rustc --explain E0412`.
2019-07-18T10:20:25.7241238Z error: Could not compile `rustc`.
2019-07-18T10:20:25.7241336Z 
2019-07-18T10:20:25.7241556Z To learn more, run the command again with --verbose.
2019-07-18T10:20:25.7268117Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-18T10:20:25.7278656Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-18T10:20:25.7278728Z Build completed unsuccessfully in 0:04:29
2019-07-18T10:20:25.7278728Z Build completed unsuccessfully in 0:04:29
2019-07-18T10:20:26.9188269Z ##[error]Bash exited with code '1'.
2019-07-18T10:20:26.9216617Z ##[section]Starting: Checkout
2019-07-18T10:20:26.9218441Z ==============================================================================
2019-07-18T10:20:26.9218498Z Task         : Get sources
2019-07-18T10:20:26.9218547Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
