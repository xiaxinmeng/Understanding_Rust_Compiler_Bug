plain
2019-07-26T10:18:02.4219151Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-26T10:18:02.4416040Z ##[command]git config gc.auto 0
2019-07-26T10:18:02.4504810Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-26T10:18:02.4559713Z ##[command]git config --get-all http.proxy
2019-07-26T10:18:02.4722950Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62969/merge:refs/remotes/pull/62969/merge
---
2019-07-26T10:18:39.0368057Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-26T10:18:39.0368093Z 
2019-07-26T10:18:39.0368908Z   git checkout -b <new-branch-name>
2019-07-26T10:18:39.0369167Z 
2019-07-26T10:18:39.0369267Z HEAD is now at e36ba20a5 Merge 8705fc9ad54c08bffc4c0ca340f3c06b637d160c into 4268e7ee22935f086b856ef0063a9e22b49aeddb
2019-07-26T10:18:39.0536726Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-26T10:18:39.0539816Z ==============================================================================
2019-07-26T10:18:39.0539879Z Task         : Bash
2019-07-26T10:18:39.0539930Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-26T10:26:15.6719767Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-07-26T10:26:17.0995053Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-07-26T10:26:18.3862625Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-07-26T10:26:31.6137688Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-07-26T10:26:35.6531247Z error[E0422]: cannot find struct, variant or union type `PointerOutOfBounds` in this scope
2019-07-26T10:26:35.6532682Z     |
2019-07-26T10:26:35.6532682Z     |
2019-07-26T10:26:35.6533143Z 437 |             PointerOutOfBounds { ptr, msg, allocation_size } => {
2019-07-26T10:26:35.6533891Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T10:26:35.6534142Z     |
2019-07-26T10:26:35.6534142Z     |
2019-07-26T10:26:35.6534488Z 1   | use crate::mir::interpret::error::UnsupportedInfo::PointerOutOfBounds;
2019-07-26T10:26:35.6535146Z 
2019-07-26T10:26:35.6535146Z 
2019-07-26T10:26:35.6665766Z error[E0531]: cannot find tuple struct/variant `NoMirFor` in this scope
2019-07-26T10:26:35.6666454Z     |
2019-07-26T10:26:35.6666454Z     |
2019-07-26T10:26:35.6666802Z 445 |             NoMirFor(ref func) => write!(f, "no mir for `{}`", func),
2019-07-26T10:26:35.6667824Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T10:26:35.6668112Z     |
2019-07-26T10:26:35.6668112Z     |
2019-07-26T10:26:35.6668516Z 1   | use crate::mir::interpret::error::UnsupportedInfo::NoMirFor;
2019-07-26T10:26:35.6668899Z 
2019-07-26T10:26:35.6668899Z 
2019-07-26T10:26:35.6807889Z error[E0531]: cannot find tuple struct/variant `FunctionAbiMismatch` in this scope
2019-07-26T10:26:35.6808555Z     |
2019-07-26T10:26:35.6808555Z     |
2019-07-26T10:26:35.6808962Z 446 |             FunctionAbiMismatch(caller_abi, callee_abi) =>
2019-07-26T10:26:35.6809659Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T10:26:35.6809923Z     |
2019-07-26T10:26:35.6809923Z     |
2019-07-26T10:26:35.6810284Z 1   | use crate::mir::interpret::error::UnsupportedInfo::FunctionAbiMismatch;
2019-07-26T10:26:35.6810584Z 
2019-07-26T10:26:35.6810584Z 
2019-07-26T10:26:35.6939439Z error[E0531]: cannot find tuple struct/variant `FunctionArgMismatch` in this scope
2019-07-26T10:26:35.6940180Z     |
2019-07-26T10:26:35.6940180Z     |
2019-07-26T10:26:35.6940520Z 449 |             FunctionArgMismatch(caller_ty, callee_ty) =>
2019-07-26T10:26:35.6941454Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T10:26:35.6941710Z     |
2019-07-26T10:26:35.6941710Z     |
2019-07-26T10:26:35.6942056Z 1   | use crate::mir::interpret::error::UnsupportedInfo::FunctionArgMismatch;
2019-07-26T10:26:35.6942356Z 
2019-07-26T10:26:35.6942356Z 
2019-07-26T10:26:35.7074027Z error[E0531]: cannot find tuple struct/variant `FunctionRetMismatch` in this scope
2019-07-26T10:26:35.7074737Z     |
2019-07-26T10:26:35.7074737Z     |
2019-07-26T10:26:35.7075071Z 453 |             FunctionRetMismatch(caller_ty, callee_ty) =>
2019-07-26T10:26:35.7076084Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T10:26:35.7076393Z     |
2019-07-26T10:26:35.7076393Z     |
2019-07-26T10:26:35.7076759Z 1   | use crate::mir::interpret::error::UnsupportedInfo::FunctionRetMismatch;
2019-07-26T10:26:35.7077037Z 
2019-07-26T10:26:35.7077037Z 
2019-07-26T10:26:35.7203019Z error[E0531]: cannot find tuple struct/variant `Layout` in this scope
2019-07-26T10:26:35.7203670Z     |
2019-07-26T10:26:35.7203994Z 470 |             Layout(ref err) =>
2019-07-26T10:26:35.7204349Z     |             ^^^^^^ not found in this scope
2019-07-26T10:26:35.7204700Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T10:26:35.7204700Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T10:26:35.7204969Z     |
2019-07-26T10:26:35.7205358Z 1   | use crate::mir::interpret::error::InvalidProgramInfo::Layout;
2019-07-26T10:26:35.7205750Z 
2019-07-26T10:26:35.7205750Z 
2019-07-26T10:26:35.7342909Z error[E0531]: cannot find tuple struct/variant `InvalidDiscriminant` in this scope
2019-07-26T10:26:35.7343622Z     |
2019-07-26T10:26:35.7343622Z     |
2019-07-26T10:26:35.7343939Z 478 |             InvalidDiscriminant(val) =>
2019-07-26T10:26:35.7344691Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T10:26:35.7344937Z     |
2019-07-26T10:26:35.7344937Z     |
2019-07-26T10:26:35.7345575Z 1   | use crate::mir::interpret::error::UnsupportedInfo::InvalidDiscriminant;
2019-07-26T10:26:35.7346420Z 
2019-07-26T10:26:35.7346420Z 
2019-07-26T10:26:35.7487369Z error[E0531]: cannot find tuple struct/variant `InvalidBoolOp` in this scope
2019-07-26T10:26:35.7488064Z     |
2019-07-26T10:26:35.7488064Z     |
2019-07-26T10:26:35.7488368Z 552 |             InvalidBoolOp(_) =>
2019-07-26T10:26:35.7489180Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T10:26:35.7489425Z     |
2019-07-26T10:26:35.7489425Z     |
2019-07-26T10:26:35.7490359Z 1   | use crate::mir::interpret::error::UnsupportedInfo::InvalidBoolOp;
2019-07-26T10:26:35.7490692Z 
2019-07-26T10:26:35.7490692Z 
2019-07-26T10:26:35.7636900Z error[E0531]: cannot find tuple struct/variant `UnterminatedCString` in this scope
2019-07-26T10:26:35.7637541Z     |
2019-07-26T10:26:35.7637541Z     |
2019-07-26T10:26:35.7638127Z 554 |             UnterminatedCString(_) =>
2019-07-26T10:26:35.7638885Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T10:26:35.7639220Z     |
2019-07-26T10:26:35.7639220Z     |
2019-07-26T10:26:35.7639769Z 1   | use crate::mir::interpret::error::UnsupportedInfo::UnterminatedCString;
2019-07-26T10:26:35.7640410Z 
2019-07-26T10:26:35.7640410Z 
2019-07-26T10:26:35.7767965Z error[E0531]: cannot find tuple struct/variant `ReadUndefBytes` in this scope
2019-07-26T10:26:35.7768668Z     |
2019-07-26T10:26:35.7768668Z     |
2019-07-26T10:26:35.7769065Z 557 |             ReadUndefBytes(_) =>
2019-07-26T10:26:35.7770087Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T10:26:35.7770436Z     |
2019-07-26T10:26:35.7770436Z     |
2019-07-26T10:26:35.7770798Z 1   | use crate::mir::interpret::error::UnsupportedInfo::ReadUndefBytes;
2019-07-26T10:26:35.7771082Z 
2019-07-26T10:26:35.7771082Z 
2019-07-26T10:26:48.4280935Z error[E0599]: no variant or associated item named `ReadUndefBytes` found for type `mir::interpret::error::InterpError<'_>` in the current scope
2019-07-26T10:26:48.4282349Z    --> src/librustc/mir/interpret/allocation.rs:519:30
2019-07-26T10:26:48.4283071Z     |
2019-07-26T10:26:48.4283787Z 519 |         ).or_else(|idx| err!(ReadUndefBytes(idx)))
2019-07-26T10:26:48.4285364Z     | 
2019-07-26T10:26:48.4286843Z    ::: src/librustc/mir/interpret/error.rs:371:1
2019-07-26T10:26:48.4287511Z     |
2019-07-26T10:26:48.4288030Z 371 | pub enum InterpError<'tcx> {
2019-07-26T10:26:48.4288030Z 371 | pub enum InterpError<'tcx> {
2019-07-26T10:26:48.4288613Z     | -------------------------- variant or associated item `ReadUndefBytes` not found here
2019-07-26T10:26:48.4288829Z 
2019-07-26T10:26:48.5281130Z error[E0599]: no variant or associated item named `Layout` found for type `mir::interpret::error::InterpError<'_>` in the current scope
2019-07-26T10:26:48.5282614Z     |
2019-07-26T10:26:48.5282614Z     |
2019-07-26T10:26:48.5283147Z 145 |             InterpError::Layout(LayoutError::SizeOverflow(_)) |
2019-07-26T10:26:48.5284295Z ...
2019-07-26T10:26:48.5284982Z 371 | pub enum InterpError<'tcx> {
2019-07-26T10:26:48.5284982Z 371 | pub enum InterpError<'tcx> {
2019-07-26T10:26:48.5285645Z     | -------------------------- variant or associated item `Layout` not found here
2019-07-26T10:26:48.5285835Z 
2019-07-26T10:26:48.8534237Z error[E0599]: no variant or associated item named `ReadPointerAsBytes` found for type `mir::interpret::error::InterpError<'_>` in the current scope
2019-07-26T10:26:48.8534801Z    --> src/librustc/mir/interpret/value.rs:363:36
2019-07-26T10:26:48.8535002Z     |
2019-07-26T10:26:48.8535273Z 363 |             Scalar::Ptr(_) => err!(ReadPointerAsBytes),
2019-07-26T10:26:48.8535836Z     | 
2019-07-26T10:26:48.8536190Z    ::: src/librustc/mir/interpret/error.rs:371:1
2019-07-26T10:26:48.8536396Z     |
2019-07-26T10:26:48.8536777Z 371 | pub enum InterpError<'tcx> {
2019-07-26T10:26:48.8536777Z 371 | pub enum InterpError<'tcx> {
2019-07-26T10:26:48.8537113Z     | -------------------------- variant or associated item `ReadPointerAsBytes` not found here
2019-07-26T10:26:48.8537167Z 
2019-07-26T10:26:48.8573438Z error[E0599]: no variant or associated item named `InvalidNullPointerUsage` found for type `mir::interpret::error::InterpError<'_>` in the current scope
2019-07-26T10:26:48.8573868Z    --> src/librustc/mir/interpret/value.rs:376:49
2019-07-26T10:26:48.8574193Z     |
2019-07-26T10:26:48.8574641Z 376 |             Scalar::Raw { data: 0, .. } => err!(InvalidNullPointerUsage),
2019-07-26T10:26:48.8575331Z     | 
2019-07-26T10:26:48.8575573Z    ::: src/librustc/mir/interpret/error.rs:371:1
2019-07-26T10:26:48.8575776Z     |
2019-07-26T10:26:48.8576040Z 371 | pub enum InterpError<'tcx> {
2019-07-26T10:26:48.8576040Z 371 | pub enum InterpError<'tcx> {
2019-07-26T10:26:48.8576603Z     | -------------------------- variant or associated item `InvalidNullPointerUsage` not found here
2019-07-26T10:26:48.8576657Z 
2019-07-26T10:26:48.8605784Z error[E0599]: no variant or associated item named `ReadBytesAsPointer` found for type `mir::interpret::error::InterpError<'_>` in the current scope
2019-07-26T10:26:48.8606062Z    --> src/librustc/mir/interpret/value.rs:377:40
2019-07-26T10:26:48.8606407Z     |
2019-07-26T10:26:48.8606822Z 377 |             Scalar::Raw { .. } => err!(ReadBytesAsPointer),
2019-07-26T10:26:48.8607447Z     | 
2019-07-26T10:26:48.8607694Z    ::: src/librustc/mir/interpret/error.rs:371:1
2019-07-26T10:26:48.8608050Z     |
2019-07-26T10:26:48.8608381Z 371 | pub enum InterpError<'tcx> {
2019-07-26T10:26:48.8608381Z 371 | pub enum InterpError<'tcx> {
2019-07-26T10:26:48.8608716Z     | -------------------------- variant or associated item `ReadBytesAsPointer` not found here
2019-07-26T10:26:48.8608861Z 
2019-07-26T10:26:48.8647742Z error[E0599]: no variant or associated item named `InvalidBool` found for type `mir::interpret::error::InterpError<'_>` in the current scope
2019-07-26T10:26:48.8648052Z    --> src/librustc/mir/interpret/value.rs:409:23
2019-07-26T10:26:48.8648301Z     |
2019-07-26T10:26:48.8648564Z 409 |             _ => err!(InvalidBool),
2019-07-26T10:26:48.8649177Z     | 
2019-07-26T10:26:48.8649846Z    ::: src/librustc/mir/interpret/error.rs:371:1
2019-07-26T10:26:48.8650148Z     |
2019-07-26T10:26:48.8650451Z 371 | pub enum InterpError<'tcx> {
2019-07-26T10:26:48.8650451Z 371 | pub enum InterpError<'tcx> {
2019-07-26T10:26:48.8650787Z     | -------------------------- variant or associated item `InvalidBool` not found here
2019-07-26T10:26:48.8650831Z 
2019-07-26T10:26:48.9004206Z error[E0599]: no variant or associated item named `ReadUndefBytes` found for type `mir::interpret::error::InterpError<'_>` in the current scope
2019-07-26T10:26:48.9004590Z    --> src/librustc/mir/interpret/value.rs:540:45
2019-07-26T10:26:48.9004877Z     |
2019-07-26T10:26:48.9005241Z 540 |             ScalarMaybeUndef::Undef => err!(ReadUndefBytes(Size::from_bytes(0))),
2019-07-26T10:26:48.9005985Z     | 
2019-07-26T10:26:48.9006275Z    ::: src/librustc/mir/interpret/error.rs:371:1
2019-07-26T10:26:48.9006518Z     |
2019-07-26T10:26:48.9006843Z 371 | pub enum InterpError<'tcx> {
2019-07-26T10:26:48.9006843Z 371 | pub enum InterpError<'tcx> {
2019-07-26T10:26:48.9007238Z     | -------------------------- variant or associated item `ReadUndefBytes` not found here
2019-07-26T10:26:48.9007292Z 
2019-07-26T10:26:48.9738452Z error[E0599]: no variant or associated item named `UnterminatedCString` found for type `mir::interpret::error::InterpError<'_>` in the current scope
2019-07-26T10:26:48.9738826Z    --> src/librustc/mir/interpret/allocation.rs:247:26
2019-07-26T10:26:48.9739131Z     |
2019-07-26T10:26:48.9739546Z 247 |             None => err!(UnterminatedCString(ptr.erase_tag())),
2019-07-26T10:26:48.9740308Z     | 
2019-07-26T10:26:48.9740767Z    ::: src/librustc/mir/interpret/error.rs:371:1
2019-07-26T10:26:48.9741549Z     |
2019-07-26T10:26:48.9741868Z 371 | pub enum InterpError<'tcx> {
2019-07-26T10:26:48.9741868Z 371 | pub enum InterpError<'tcx> {
2019-07-26T10:26:48.9742259Z     | -------------------------- variant or associated item `UnterminatedCString` not found here
2019-07-26T10:26:48.9742460Z 
2019-07-26T10:26:48.9917816Z error[E0599]: no variant or associated item named `ReadPointerAsBytes` found for type `mir::interpret::error::InterpError<'_>` in the current scope
2019-07-26T10:26:48.9918211Z    --> src/librustc/mir/interpret/allocation.rs:449:18
2019-07-26T10:26:48.9918470Z     |
2019-07-26T10:26:48.9918793Z 449 |             err!(ReadPointerAsBytes)
2019-07-26T10:26:48.9919487Z     | 
2019-07-26T10:26:48.9919772Z    ::: src/librustc/mir/interpret/error.rs:371:1
2019-07-26T10:26:48.9920568Z     |
2019-07-26T10:26:48.9920870Z 371 | pub enum InterpError<'tcx> {
2019-07-26T10:26:48.9920870Z 371 | pub enum InterpError<'tcx> {
2019-07-26T10:26:48.9921258Z     | -------------------------- variant or associated item `ReadPointerAsBytes` not found here
2019-07-26T10:26:48.9921324Z 
2019-07-26T10:26:49.0793232Z error: no variant `PointerOutOfBounds` in enum `mir::interpret::error::InterpError<'_>`
2019-07-26T10:26:49.0793602Z    --> src/librustc/mir/interpret/pointer.rs:201:18
2019-07-26T10:26:49.0793877Z     |
2019-07-26T10:26:49.0794191Z 201 |             err!(PointerOutOfBounds {
2019-07-26T10:26:49.0794770Z     | 
2019-07-26T10:26:49.0795061Z    ::: src/librustc/mir/interpret/error.rs:371:1
2019-07-26T10:26:49.0795298Z     |
2019-07-26T10:26:49.0795608Z 371 | pub enum InterpError<'tcx> {
2019-07-26T10:26:49.0795608Z 371 | pub enum InterpError<'tcx> {
2019-07-26T10:26:49.0796002Z     | -------------------------- variant `PointerOutOfBounds` not found here
2019-07-26T10:26:49.0796558Z    ::: src/librustc/mir/interpret/mod.rs:5:27
2019-07-26T10:26:49.0796796Z     |
2019-07-26T10:26:49.0796796Z     |
2019-07-26T10:26:49.0797145Z 5   |     ($($tt:tt)*) => { Err($crate::mir::interpret::InterpError::$($tt)*.into()) };
2019-07-26T10:26:49.0797581Z     |                           ------ variant not found in `mir::interpret::error::InterpError<'_>`
2019-07-26T10:26:57.2458566Z error: aborting due to 20 previous errors
2019-07-26T10:26:57.2459183Z 
2019-07-26T10:26:57.2459806Z Some errors have detailed explanations: E0422, E0599.
2019-07-26T10:26:57.2460478Z For more information about an error, try `rustc --explain E0422`.
2019-07-26T10:26:57.2460478Z For more information about an error, try `rustc --explain E0422`.
2019-07-26T10:26:57.4291855Z error: Could not compile `rustc`.
2019-07-26T10:26:57.4291974Z 
2019-07-26T10:26:57.4292968Z To learn more, run the command again with --verbose.
2019-07-26T10:26:57.4316356Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-26T10:26:57.4331306Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-26T10:26:57.4331390Z Build completed unsuccessfully in 0:05:06
2019-07-26T10:26:57.4331390Z Build completed unsuccessfully in 0:05:06
2019-07-26T10:26:58.0353855Z ##[error]Bash exited with code '1'.
2019-07-26T10:26:58.0393541Z ##[section]Starting: Checkout
2019-07-26T10:26:58.0395351Z ==============================================================================
2019-07-26T10:26:58.0395414Z Task         : Get sources
2019-07-26T10:26:58.0395467Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
