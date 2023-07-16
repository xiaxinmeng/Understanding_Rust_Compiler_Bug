plain
2019-07-18T10:12:14.3535416Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-18T10:12:14.3733242Z ##[command]git config gc.auto 0
2019-07-18T10:12:14.3821425Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-18T10:12:14.3874505Z ##[command]git config --get-all http.proxy
2019-07-18T10:12:14.4017512Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/60951/merge:refs/remotes/pull/60951/merge
---
2019-07-18T10:12:49.2456190Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-18T10:12:49.2457890Z 
2019-07-18T10:12:49.2458797Z   git checkout -b <new-branch-name>
2019-07-18T10:12:49.2458867Z 
2019-07-18T10:12:49.2458952Z HEAD is now at 28590e917 Merge 589f6a7dc66d88739485a1f9044ab29c166420a0 into 2c3b05d90d4dee879a0d9f33e721c930b087d7d1
2019-07-18T10:12:49.2607094Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-18T10:12:49.2610688Z ==============================================================================
2019-07-18T10:12:49.2610800Z Task         : Bash
2019-07-18T10:12:49.2610855Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-18T10:20:40.4498512Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-07-18T10:20:42.0285178Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-07-18T10:20:43.3643614Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-07-18T10:20:57.0643450Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-07-18T10:21:01.1232478Z error[E0412]: cannot find type `FnSig` in this scope
2019-07-18T10:21:01.1234119Z    --> src/librustc/mir/interpret/error.rs:250:31
2019-07-18T10:21:01.1234836Z     |
2019-07-18T10:21:01.1235619Z 250 |     FunctionPointerTyMismatch(FnSig<'tcx>, FnSig<'tcx>),
2019-07-18T10:21:01.1237194Z help: possible candidate is found in another module, you can import it into scope
2019-07-18T10:21:01.1237847Z     |
2019-07-18T10:21:01.1237847Z     |
2019-07-18T10:21:01.1238620Z 1   | use crate::ty::sty::FnSig;
2019-07-18T10:21:01.1239590Z 
2019-07-18T10:21:01.1239590Z 
2019-07-18T10:21:01.1367208Z error[E0412]: cannot find type `FnSig` in this scope
2019-07-18T10:21:01.1368955Z    --> src/librustc/mir/interpret/error.rs:250:44
2019-07-18T10:21:01.1370361Z     |
2019-07-18T10:21:01.1371231Z 250 |     FunctionPointerTyMismatch(FnSig<'tcx>, FnSig<'tcx>),
2019-07-18T10:21:01.1373048Z help: possible candidate is found in another module, you can import it into scope
2019-07-18T10:21:01.1373577Z     |
2019-07-18T10:21:01.1373577Z     |
2019-07-18T10:21:01.1374122Z 1   | use crate::ty::sty::FnSig;
2019-07-18T10:21:01.1375026Z 
2019-07-18T10:21:01.1375026Z 
2019-07-18T10:21:01.1493838Z error[E0412]: cannot find type `AccessKind` in this scope
2019-07-18T10:21:01.1494624Z    --> src/librustc/mir/interpret/error.rs:291:17
2019-07-18T10:21:01.1495153Z     |
2019-07-18T10:21:01.1495636Z 291 |         access: AccessKind,
2019-07-18T10:21:01.1496431Z 
2019-07-18T10:21:01.1496431Z 
2019-07-18T10:21:01.1642214Z error[E0412]: cannot find type `Lock` in this scope
2019-07-18T10:21:01.1643136Z    --> src/librustc/mir/interpret/error.rs:292:15
2019-07-18T10:21:01.1644171Z 292 |         lock: Lock,
2019-07-18T10:21:01.1644840Z     |               ^^^^ not found in this scope
2019-07-18T10:21:01.1644840Z     |               ^^^^ not found in this scope
2019-07-18T10:21:01.1645399Z help: possible candidates are found in other modules, you can import them into scope
2019-07-18T10:21:01.1646454Z 1   | use rustc_data_structures::flock::Lock;
2019-07-18T10:21:01.1646938Z     |
2019-07-18T10:21:01.1647515Z 1   | use rustc_data_structures::sync::Lock;
2019-07-18T10:21:01.1647987Z     |
2019-07-18T10:21:01.1647987Z     |
2019-07-18T10:21:01.1648182Z 
2019-07-18T10:21:01.1782014Z error[E0412]: cannot find type `AccessKind` in this scope
2019-07-18T10:21:01.1782855Z    --> src/librustc/mir/interpret/error.rs:297:15
2019-07-18T10:21:01.1783472Z     |
2019-07-18T10:21:01.1784025Z 297 |         kind: AccessKind,
2019-07-18T10:21:01.1784913Z 
2019-07-18T10:21:01.1784913Z 
2019-07-18T10:21:01.1929723Z error[E0412]: cannot find type `Lock` in this scope
2019-07-18T10:21:01.1930859Z    --> src/librustc/mir/interpret/error.rs:298:15
2019-07-18T10:21:01.1931979Z 298 |         lock: Lock,
2019-07-18T10:21:01.1932596Z     |               ^^^^ not found in this scope
2019-07-18T10:21:01.1932596Z     |               ^^^^ not found in this scope
2019-07-18T10:21:01.1933197Z help: possible candidates are found in other modules, you can import them into scope
2019-07-18T10:21:01.1934311Z 1   | use rustc_data_structures::flock::Lock;
2019-07-18T10:21:01.1934820Z     |
2019-07-18T10:21:01.1935374Z 1   | use rustc_data_structures::sync::Lock;
2019-07-18T10:21:01.1936154Z     |
2019-07-18T10:21:01.1936154Z     |
2019-07-18T10:21:01.1936965Z 
2019-07-18T10:21:01.2064355Z error[E0412]: cannot find type `Lock` in this scope
2019-07-18T10:21:01.2064844Z    --> src/librustc/mir/interpret/error.rs:304:15
2019-07-18T10:21:01.2068923Z 304 |         lock: Lock,
2019-07-18T10:21:01.2070413Z     |               ^^^^ not found in this scope
2019-07-18T10:21:01.2070413Z     |               ^^^^ not found in this scope
2019-07-18T10:21:01.2071143Z help: possible candidates are found in other modules, you can import them into scope
2019-07-18T10:21:01.2072520Z 1   | use rustc_data_structures::flock::Lock;
2019-07-18T10:21:01.2073144Z     |
2019-07-18T10:21:01.2073871Z 1   | use rustc_data_structures::sync::Lock;
2019-07-18T10:21:01.2074474Z     |
2019-07-18T10:21:01.2074474Z     |
2019-07-18T10:21:01.2074774Z 
2019-07-18T10:21:01.2216015Z error[E0412]: cannot find type `Lock` in this scope
2019-07-18T10:21:01.2216417Z    --> src/librustc/mir/interpret/error.rs:308:15
2019-07-18T10:21:01.2216998Z 308 |         lock: Lock,
2019-07-18T10:21:01.2217324Z     |               ^^^^ not found in this scope
2019-07-18T10:21:01.2217324Z     |               ^^^^ not found in this scope
2019-07-18T10:21:01.2217642Z help: possible candidates are found in other modules, you can import them into scope
2019-07-18T10:21:01.2218172Z 1   | use rustc_data_structures::flock::Lock;
2019-07-18T10:21:01.2218950Z     |
2019-07-18T10:21:01.2219264Z 1   | use rustc_data_structures::sync::Lock;
2019-07-18T10:21:01.2219496Z     |
2019-07-18T10:21:01.2219496Z     |
2019-07-18T10:21:01.2219550Z 
2019-07-18T10:21:02.1208192Z error: unused import: `syntax::symbol::Symbol`
2019-07-18T10:21:02.1208645Z   --> src/librustc/mir/interpret/error.rs:19:5
2019-07-18T10:21:02.1209517Z    |
2019-07-18T10:21:02.1209882Z 19 | use syntax::symbol::Symbol;
2019-07-18T10:21:02.1210205Z    |     ^^^^^^^^^^^^^^^^^^^^^^
2019-07-18T10:21:02.1210532Z    |
2019-07-18T10:21:02.1210877Z    = note: `-D unused-imports` implied by `-D warnings`
2019-07-18T10:21:02.1210932Z 
2019-07-18T10:21:03.4631357Z error: lifetime parameter `'tcx` never used
2019-07-18T10:21:03.4631875Z    --> src/librustc/mir/interpret/error.rs:344:25
2019-07-18T10:21:03.4632151Z     |
2019-07-18T10:21:03.4632525Z 344 | pub enum EvalErrorPanic<'tcx, O> {
2019-07-18T10:21:03.4632906Z     |                         ^^^^--
2019-07-18T10:21:03.4633243Z     |                         |
2019-07-18T10:21:03.4633631Z     |                         help: elide the unused lifetime
2019-07-18T10:21:03.4634438Z note: lint level defined here
2019-07-18T10:21:03.4634735Z    --> src/librustc/lib.rs:32:9
2019-07-18T10:21:03.4635022Z     |
2019-07-18T10:21:03.4635022Z     |
2019-07-18T10:21:03.4635339Z 32  | #![deny(unused_lifetimes)]
2019-07-18T10:21:03.4635887Z 
2019-07-18T10:21:03.9823108Z error: aborting due to 10 previous errors
2019-07-18T10:21:03.9823233Z 
2019-07-18T10:21:03.9823580Z For more information about this error, try `rustc --explain E0412`.
2019-07-18T10:21:03.9823580Z For more information about this error, try `rustc --explain E0412`.
2019-07-18T10:21:04.1470923Z error: Could not compile `rustc`.
2019-07-18T10:21:04.1471659Z 
2019-07-18T10:21:04.1472548Z To learn more, run the command again with --verbose.
2019-07-18T10:21:04.1503437Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-18T10:21:04.1508639Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-18T10:21:04.1508726Z Build completed unsuccessfully in 0:05:03
2019-07-18T10:21:04.1508726Z Build completed unsuccessfully in 0:05:03
2019-07-18T10:21:05.2986276Z ##[error]Bash exited with code '1'.
2019-07-18T10:21:05.3031415Z ##[section]Starting: Checkout
2019-07-18T10:21:05.3033316Z ==============================================================================
2019-07-18T10:21:05.3033400Z Task         : Get sources
2019-07-18T10:21:05.3033456Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
