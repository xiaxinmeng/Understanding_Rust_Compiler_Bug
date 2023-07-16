plain
2019-07-26T13:50:18.7548120Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-26T13:50:18.7750120Z ##[command]git config gc.auto 0
2019-07-26T13:50:18.7829088Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-26T13:50:18.7919387Z ##[command]git config --get-all http.proxy
2019-07-26T13:50:18.8020474Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62969/merge:refs/remotes/pull/62969/merge
---
2019-07-26T13:50:51.9707253Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-26T13:50:51.9707282Z 
2019-07-26T13:50:51.9707457Z   git checkout -b <new-branch-name>
2019-07-26T13:50:51.9707483Z 
2019-07-26T13:50:51.9707525Z HEAD is now at 3785440eb Merge 186230e267a1c90400c405180918195b8fbf8add into 1a563362865e6051d4c350544131228e8eff5138
2019-07-26T13:50:51.9855525Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-26T13:50:51.9857970Z ==============================================================================
2019-07-26T13:50:51.9858018Z Task         : Bash
2019-07-26T13:50:51.9858057Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-26T13:58:10.9896768Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-07-26T13:58:12.4549234Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-07-26T13:58:13.7330140Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-07-26T13:58:27.3162156Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-07-26T13:58:32.0863057Z error[E0422]: cannot find struct, variant or union type `PointerOutOfBounds` in this scope
2019-07-26T13:58:32.0864834Z     |
2019-07-26T13:58:32.0864834Z     |
2019-07-26T13:58:32.0865507Z 435 |             PointerOutOfBounds { ptr, msg, allocation_size } => {
2019-07-26T13:58:32.0866926Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T13:58:32.0867532Z     |
2019-07-26T13:58:32.0867532Z     |
2019-07-26T13:58:32.0868519Z 1   | use crate::mir::interpret::error::UnsupportedInfo::PointerOutOfBounds;
2019-07-26T13:58:32.0869535Z 
2019-07-26T13:58:32.0869535Z 
2019-07-26T13:58:32.0994531Z error[E0531]: cannot find tuple struct/variant `ValidationFailure` in this scope
2019-07-26T13:58:32.0996360Z     |
2019-07-26T13:58:32.0997024Z 440 |             ValidationFailure(ref err) => {
2019-07-26T13:58:32.0997518Z     |             ^^^^^^^^^^^^^^^^^ not found in this scope
2019-07-26T13:58:32.0997942Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T13:58:32.0997942Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T13:58:32.0998655Z     |
2019-07-26T13:58:32.0999346Z 1   | use crate::mir::interpret::error::UnsupportedInfo::ValidationFailure;
2019-07-26T13:58:32.0999898Z 
2019-07-26T13:58:32.0999898Z 
2019-07-26T13:58:32.1123926Z error[E0531]: cannot find tuple struct/variant `NoMirFor` in this scope
2019-07-26T13:58:32.1125676Z     |
2019-07-26T13:58:32.1125676Z     |
2019-07-26T13:58:32.1126121Z 443 |             NoMirFor(ref func) => write!(f, "no mir for `{}`", func),
2019-07-26T13:58:32.1127046Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T13:58:32.1127447Z     |
2019-07-26T13:58:32.1127447Z     |
2019-07-26T13:58:32.1127935Z 1   | use crate::mir::interpret::error::UnsupportedInfo::NoMirFor;
2019-07-26T13:58:32.1128660Z 
2019-07-26T13:58:32.1128660Z 
2019-07-26T13:58:32.1243875Z error[E0531]: cannot find tuple struct/variant `FunctionAbiMismatch` in this scope
2019-07-26T13:58:33.0578391Z     |
2019-07-26T13:58:33.0578391Z     |
2019-07-26T13:58:33.0578704Z 444 |             FunctionAbiMismatch(caller_abi, callee_abi) =>
2019-07-26T13:58:33.0579467Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T13:58:33.0579715Z     |
2019-07-26T13:58:33.0579715Z     |
2019-07-26T13:58:33.0580008Z 1   | use crate::mir::interpret::error::UnsupportedInfo::FunctionAbiMismatch;
2019-07-26T13:58:33.0580245Z 
2019-07-26T13:58:33.0580245Z 
2019-07-26T13:58:33.0580526Z error[E0531]: cannot find tuple struct/variant `FunctionArgMismatch` in this scope
2019-07-26T13:58:33.0580973Z     |
2019-07-26T13:58:33.0580973Z     |
2019-07-26T13:58:33.0581266Z 447 |             FunctionArgMismatch(caller_ty, callee_ty) =>
2019-07-26T13:58:33.0582417Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T13:58:33.0582703Z     |
2019-07-26T13:58:33.0582703Z     |
2019-07-26T13:58:33.0583030Z 1   | use crate::mir::interpret::error::UnsupportedInfo::FunctionArgMismatch;
2019-07-26T13:58:33.0583320Z 
2019-07-26T13:58:33.0583320Z 
2019-07-26T13:58:33.0583620Z error[E0531]: cannot find tuple struct/variant `FunctionRetMismatch` in this scope
2019-07-26T13:58:33.0584149Z     |
2019-07-26T13:58:33.0584149Z     |
2019-07-26T13:58:33.0584458Z 451 |             FunctionRetMismatch(caller_ty, callee_ty) =>
2019-07-26T13:58:33.0585283Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T13:58:33.0585514Z     |
2019-07-26T13:58:33.0585514Z     |
2019-07-26T13:58:33.0586039Z 1   | use crate::mir::interpret::error::UnsupportedInfo::FunctionRetMismatch;
2019-07-26T13:58:33.0586499Z 
2019-07-26T13:58:33.0586499Z 
2019-07-26T13:58:33.0586767Z error[E0531]: cannot find tuple struct/variant `ReallocatedWrongMemoryKind` in this scope
2019-07-26T13:58:33.0587399Z     |
2019-07-26T13:58:33.0587399Z     |
2019-07-26T13:58:33.0587662Z 457 |             ReallocatedWrongMemoryKind(ref old, ref new) =>
2019-07-26T13:58:33.0588700Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T13:58:33.0588909Z     |
2019-07-26T13:58:33.0588909Z     |
2019-07-26T13:58:33.0589241Z 1   | use crate::mir::interpret::error::UnsupportedInfo::ReallocatedWrongMemoryKind;
2019-07-26T13:58:33.0589463Z 
2019-07-26T13:58:33.0589463Z 
2019-07-26T13:58:33.0589764Z error[E0531]: cannot find tuple struct/variant `DeallocatedWrongMemoryKind` in this scope
2019-07-26T13:58:33.0590199Z     |
2019-07-26T13:58:33.0590199Z     |
2019-07-26T13:58:33.0590677Z 459 |             DeallocatedWrongMemoryKind(ref old, ref new) =>
2019-07-26T13:58:33.0591216Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T13:58:33.0591460Z     |
2019-07-26T13:58:33.0591460Z     |
2019-07-26T13:58:33.0591791Z 1   | use crate::mir::interpret::error::UnsupportedInfo::DeallocatedWrongMemoryKind;
2019-07-26T13:58:33.0592266Z 
2019-07-26T13:58:33.0592266Z 
2019-07-26T13:58:33.0592555Z error[E0531]: cannot find tuple struct/variant `InvalidChar` in this scope
2019-07-26T13:58:33.0593141Z     |
2019-07-26T13:58:33.0593141Z     |
2019-07-26T13:58:33.0629581Z 461 |             InvalidChar(c) =>
2019-07-26T13:58:33.0630211Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T13:58:33.0630460Z     |
2019-07-26T13:58:33.0630460Z     |
2019-07-26T13:58:33.0630769Z 1   | use crate::mir::interpret::error::UnsupportedInfo::InvalidChar;
2019-07-26T13:58:33.0631222Z 
2019-07-26T13:58:33.0631222Z 
2019-07-26T13:58:33.0631969Z error[E0422]: cannot find struct, variant or union type `AlignmentCheckFailed` in this scope
2019-07-26T13:58:33.0632504Z     |
2019-07-26T13:58:33.0632504Z     |
2019-07-26T13:58:33.0632798Z 463 |             AlignmentCheckFailed { required, has } =>
2019-07-26T13:58:33.0633443Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T13:58:33.0633661Z     |
2019-07-26T13:58:33.0633661Z     |
2019-07-26T13:58:33.0633987Z 1   | use crate::mir::interpret::error::UnsupportedInfo::AlignmentCheckFailed;
2019-07-26T13:58:33.0634255Z 
2019-07-26T13:58:33.0634255Z 
2019-07-26T13:58:33.0634543Z error[E0531]: cannot find tuple struct/variant `TypeNotPrimitive` in this scope
2019-07-26T13:58:33.0635223Z     |
2019-07-26T13:58:33.0635223Z     |
2019-07-26T13:58:33.0635481Z 466 |             TypeNotPrimitive(ty) =>
2019-07-26T13:58:33.0636069Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T13:58:33.0636290Z     |
2019-07-26T13:58:33.0636290Z     |
2019-07-26T13:58:33.0636606Z 1   | use crate::mir::interpret::error::UnsupportedInfo::TypeNotPrimitive;
2019-07-26T13:58:33.0636869Z 
2019-07-26T13:58:33.0636869Z 
2019-07-26T13:58:33.0637158Z error[E0531]: cannot find tuple struct/variant `Layout` in this scope
2019-07-26T13:58:33.0637643Z     |
2019-07-26T13:58:33.0637924Z 468 |             Layout(ref err) =>
2019-07-26T13:58:33.0638242Z     |             ^^^^^^ not found in this scope
2019-07-26T13:58:33.0638532Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T13:58:33.0638532Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T13:58:33.0638761Z     |
2019-07-26T13:58:33.0639069Z 1   | use crate::mir::interpret::error::InvalidProgramInfo::Layout;
2019-07-26T13:58:33.0639334Z 
2019-07-26T13:58:33.0639334Z 
2019-07-26T13:58:33.0639717Z error[E0531]: cannot find tuple struct/variant `PathNotFound` in this scope
2019-07-26T13:58:33.0640251Z     |
2019-07-26T13:58:33.0640251Z     |
2019-07-26T13:58:33.0640532Z 470 |             PathNotFound(ref path) =>
2019-07-26T13:58:33.0641155Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T13:58:33.0641469Z     |
2019-07-26T13:58:33.0641469Z     |
2019-07-26T13:58:33.0642155Z 1   | use crate::mir::interpret::error::UnsupportedInfo::PathNotFound;
2019-07-26T13:58:33.0642452Z 
2019-07-26T13:58:33.0642452Z 
2019-07-26T13:58:33.0642750Z error[E0531]: cannot find tuple struct/variant `IncorrectAllocationInformation` in this scope
2019-07-26T13:58:33.0643250Z     |
2019-07-26T13:58:33.0643250Z     |
2019-07-26T13:58:33.0643562Z 472 |             IncorrectAllocationInformation(size, size2, align, align2) =>
2019-07-26T13:58:33.0644224Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T13:58:33.0644458Z     |
2019-07-26T13:58:33.0644458Z     |
2019-07-26T13:58:33.0644823Z 1   | use crate::mir::interpret::error::UnsupportedInfo::IncorrectAllocationInformation;
2019-07-26T13:58:33.0645103Z 
2019-07-26T13:58:33.0645103Z 
2019-07-26T13:58:33.0645566Z error[E0531]: cannot find tuple struct/variant `InvalidDiscriminant` in this scope
2019-07-26T13:58:33.0646017Z     |
2019-07-26T13:58:33.0646017Z     |
2019-07-26T13:58:33.0646281Z 476 |             InvalidDiscriminant(val) =>
2019-07-26T13:58:33.0646870Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T13:58:33.0647102Z     |
2019-07-26T13:58:33.0647102Z     |
2019-07-26T13:58:33.0647413Z 1   | use crate::mir::interpret::error::UnsupportedInfo::InvalidDiscriminant;
2019-07-26T13:58:33.0647663Z 
2019-07-26T13:58:33.0647663Z 
2019-07-26T13:58:33.0647949Z error[E0531]: cannot find tuple struct/variant `InvalidBoolOp` in this scope
2019-07-26T13:58:33.0648571Z     |
2019-07-26T13:58:33.0648571Z     |
2019-07-26T13:58:33.0648874Z 550 |             InvalidBoolOp(_) =>
2019-07-26T13:58:33.0649605Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T13:58:33.0649863Z     |
2019-07-26T13:58:33.0649863Z     |
2019-07-26T13:58:33.0650169Z 1   | use crate::mir::interpret::error::UnsupportedInfo::InvalidBoolOp;
2019-07-26T13:58:33.0650438Z 
2019-07-26T13:58:33.0650438Z 
2019-07-26T13:58:33.0650761Z error[E0531]: cannot find tuple struct/variant `UnterminatedCString` in this scope
2019-07-26T13:58:33.0651269Z     |
2019-07-26T13:58:33.0651269Z     |
2019-07-26T13:58:33.0651546Z 552 |             UnterminatedCString(_) =>
2019-07-26T13:58:33.0652859Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T13:58:33.0653097Z     |
2019-07-26T13:58:33.0653097Z     |
2019-07-26T13:58:33.0653431Z 1   | use crate::mir::interpret::error::UnsupportedInfo::UnterminatedCString;
2019-07-26T13:58:33.0653721Z 
2019-07-26T13:58:33.0653721Z 
2019-07-26T13:58:33.0654019Z error[E0531]: cannot find tuple struct/variant `ReadUndefBytes` in this scope
2019-07-26T13:58:33.0654556Z     |
2019-07-26T13:58:33.0654556Z     |
2019-07-26T13:58:33.0654861Z 555 |             ReadUndefBytes(_) =>
2019-07-26T13:58:33.0655709Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T13:58:33.0655925Z     |
2019-07-26T13:58:33.0655925Z     |
2019-07-26T13:58:33.0656240Z 1   | use crate::mir::interpret::error::UnsupportedInfo::ReadUndefBytes;
2019-07-26T13:58:33.0656491Z 
2019-07-26T13:58:33.0656491Z 
2019-07-26T13:58:33.0656790Z error[E0531]: cannot find tuple struct/variant `HeapAllocNonPowerOfTwoAlignment` in this scope
2019-07-26T13:58:33.0657287Z     |
2019-07-26T13:58:33.0657287Z     |
2019-07-26T13:58:33.0657585Z 557 |             HeapAllocNonPowerOfTwoAlignment(_) =>
2019-07-26T13:58:33.0658224Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T13:58:33.0658454Z     |
2019-07-26T13:58:33.0658454Z     |
2019-07-26T13:58:33.0658772Z 1   | use crate::mir::interpret::error::UnsupportedInfo::HeapAllocNonPowerOfTwoAlignment;
2019-07-26T13:58:33.0659032Z 
2019-07-26T13:58:33.0659032Z 
2019-07-26T13:58:33.0659310Z error[E0531]: cannot find tuple struct/variant `MachineError` in this scope
2019-07-26T13:58:33.0659811Z     |
2019-07-26T13:58:33.0659811Z     |
2019-07-26T13:58:33.0660302Z 560 |             MachineError(ref msg) |
2019-07-26T13:58:33.0660905Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T13:58:33.0661115Z     |
2019-07-26T13:58:33.0661115Z     |
2019-07-26T13:58:33.0661409Z 1   | use crate::mir::interpret::error::UnsupportedInfo::MachineError;
2019-07-26T13:58:33.0661669Z 
2019-07-26T13:58:33.0661669Z 
2019-07-26T13:58:33.0662309Z error[E0531]: cannot find tuple struct/variant `Unimplemented` in this scope
2019-07-26T13:58:33.0662937Z     |
2019-07-26T13:58:33.0662937Z     |
2019-07-26T13:58:33.0663221Z 561 |             Unimplemented(ref msg) |
2019-07-26T13:58:33.0663856Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T13:58:33.0664087Z     |
2019-07-26T13:58:33.0664087Z     |
2019-07-26T13:58:33.0664433Z 1   | use crate::mir::interpret::error::UnsupportedInfo::Unimplemented;
2019-07-26T13:58:33.0664708Z 
2019-07-26T13:58:33.0664708Z 
2019-07-26T13:58:33.0665015Z error[E0531]: cannot find tuple struct/variant `AbiViolation` in this scope
2019-07-26T13:58:33.0665763Z     |
2019-07-26T13:58:33.0665763Z     |
2019-07-26T13:58:33.0666011Z 562 |             AbiViolation(ref msg) |
2019-07-26T13:58:33.0666571Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T13:58:33.0666782Z     |
2019-07-26T13:58:33.0666782Z     |
2019-07-26T13:58:33.0667078Z 1   | use crate::mir::interpret::error::UnsupportedInfo::AbiViolation;
2019-07-26T13:58:33.0667328Z 
2019-07-26T13:58:33.0667328Z 
2019-07-26T13:58:33.0667603Z error[E0531]: cannot find tuple struct/variant `Intrinsic` in this scope
2019-07-26T13:58:33.0668080Z     |
2019-07-26T13:58:33.0668080Z     |
2019-07-26T13:58:33.0668356Z 563 |             Intrinsic(ref msg) =>
2019-07-26T13:58:33.0668942Z help: possible candidates are found in other modules, you can import them into scope
2019-07-26T13:58:33.0669164Z     |
2019-07-26T13:58:33.0669164Z     |
2019-07-26T13:58:33.0669452Z 1   | use crate::mir::interpret::error::UnsupportedInfo::Intrinsic;
2019-07-26T13:58:33.0669673Z     |
2019-07-26T13:58:33.0669957Z 1   | use crate::ty::instance::InstanceDef::Intrinsic;
2019-07-26T13:58:33.0670271Z 
2019-07-26T13:58:33.0670271Z 
2019-07-26T13:58:45.4948369Z error[E0599]: no variant or associated item named `ReadUndefBytes` found for type `mir::interpret::error::InterpError<'_>` in the current scope
2019-07-26T13:58:45.4950487Z    --> src/librustc/mir/interpret/allocation.rs:519:30
2019-07-26T13:58:45.4951391Z     |
2019-07-26T13:58:45.4952555Z 519 |         ).or_else(|idx| err!(ReadUndefBytes(idx)))
2019-07-26T13:58:45.4954284Z     | 
2019-07-26T13:58:45.4954889Z    ::: src/librustc/mir/interpret/error.rs:404:1
2019-07-26T13:58:45.4955614Z     |
2019-07-26T13:58:45.4956134Z 404 | pub enum InterpError<'tcx> {
2019-07-26T13:58:45.4956134Z 404 | pub enum InterpError<'tcx> {
2019-07-26T13:58:45.4956843Z     | -------------------------- variant or associated item `ReadUndefBytes` not found here
2019-07-26T13:58:45.4957082Z 
2019-07-26T13:58:45.6018742Z error[E0599]: no variant or associated item named `Layout` found for type `mir::interpret::error::InterpError<'_>` in the current scope
2019-07-26T13:58:45.6020131Z     |
2019-07-26T13:58:45.6020131Z     |
2019-07-26T13:58:45.6020637Z 145 |             InterpError::Layout(LayoutError::SizeOverflow(_)) |
2019-07-26T13:58:45.6021994Z ...
2019-07-26T13:58:45.6022877Z 404 | pub enum InterpError<'tcx> {
2019-07-26T13:58:45.6022877Z 404 | pub enum InterpError<'tcx> {
2019-07-26T13:58:45.6023549Z     | -------------------------- variant or associated item `Layout` not found here
2019-07-26T13:58:45.6023828Z 
2019-07-26T13:58:45.9570770Z error[E0599]: no variant or associated item named `ReadPointerAsBytes` found for type `mir::interpret::error::InterpError<'_>` in the current scope
2019-07-26T13:58:45.9571160Z    --> src/librustc/mir/interpret/value.rs:363:36
2019-07-26T13:58:45.9571422Z     |
2019-07-26T13:58:45.9578097Z 363 |             Scalar::Ptr(_) => err!(ReadPointerAsBytes),
2019-07-26T13:58:45.9579169Z     | 
2019-07-26T13:58:45.9579569Z    ::: src/librustc/mir/interpret/error.rs:404:1
2019-07-26T13:58:45.9579915Z     |
2019-07-26T13:58:45.9580295Z 404 | pub enum InterpError<'tcx> {
2019-07-26T13:58:45.9580295Z 404 | pub enum InterpError<'tcx> {
2019-07-26T13:58:45.9580759Z     | -------------------------- variant or associated item `ReadPointerAsBytes` not found here
2019-07-26T13:58:45.9580936Z 
2019-07-26T13:58:45.9609342Z error[E0599]: no variant or associated item named `InvalidNullPointerUsage` found for type `mir::interpret::error::InterpError<'_>` in the current scope
2019-07-26T13:58:45.9609675Z    --> src/librustc/mir/interpret/value.rs:376:49
2019-07-26T13:58:45.9609893Z     |
2019-07-26T13:58:45.9610147Z 376 |             Scalar::Raw { data: 0, .. } => err!(InvalidNullPointerUsage),
2019-07-26T13:58:45.9610823Z     | 
2019-07-26T13:58:45.9611035Z    ::: src/librustc/mir/interpret/error.rs:404:1
2019-07-26T13:58:45.9611231Z     |
2019-07-26T13:58:45.9611462Z 404 | pub enum InterpError<'tcx> {
2019-07-26T13:58:45.9611462Z 404 | pub enum InterpError<'tcx> {
2019-07-26T13:58:45.9611750Z     | -------------------------- variant or associated item `InvalidNullPointerUsage` not found here
2019-07-26T13:58:45.9611808Z 
2019-07-26T13:58:45.9636790Z error[E0599]: no variant or associated item named `ReadBytesAsPointer` found for type `mir::interpret::error::InterpError<'_>` in the current scope
2019-07-26T13:58:45.9637076Z    --> src/librustc/mir/interpret/value.rs:377:40
2019-07-26T13:58:45.9637254Z     |
2019-07-26T13:58:45.9637505Z 377 |             Scalar::Raw { .. } => err!(ReadBytesAsPointer),
2019-07-26T13:58:45.9638039Z     | 
2019-07-26T13:58:45.9638244Z    ::: src/librustc/mir/interpret/error.rs:404:1
2019-07-26T13:58:45.9638444Z     |
2019-07-26T13:58:45.9638656Z 404 | pub enum InterpError<'tcx> {
2019-07-26T13:58:45.9638656Z 404 | pub enum InterpError<'tcx> {
2019-07-26T13:58:45.9638945Z     | -------------------------- variant or associated item `ReadBytesAsPointer` not found here
2019-07-26T13:58:45.9638982Z 
2019-07-26T13:58:45.9675380Z error[E0599]: no variant or associated item named `InvalidBool` found for type `mir::interpret::error::InterpError<'_>` in the current scope
2019-07-26T13:58:45.9675766Z    --> src/librustc/mir/interpret/value.rs:409:23
2019-07-26T13:58:45.9676308Z     |
2019-07-26T13:58:45.9676908Z 409 |             _ => err!(InvalidBool),
2019-07-26T13:58:45.9677406Z     | 
2019-07-26T13:58:45.9677608Z    ::: src/librustc/mir/interpret/error.rs:404:1
2019-07-26T13:58:45.9677797Z     |
2019-07-26T13:58:45.9678015Z 404 | pub enum InterpError<'tcx> {
2019-07-26T13:58:45.9678015Z 404 | pub enum InterpError<'tcx> {
2019-07-26T13:58:45.9678573Z     | -------------------------- variant or associated item `InvalidBool` not found here
2019-07-26T13:58:45.9678639Z 
2019-07-26T13:58:45.9713983Z error[E0599]: no variant or associated item named `InvalidChar` found for type `mir::interpret::error::InterpError<'_>` in the current scope
2019-07-26T13:58:45.9714320Z    --> src/librustc/mir/interpret/value.rs:417:26
2019-07-26T13:58:45.9714582Z     |
2019-07-26T13:58:45.9714880Z 417 |             None => err!(InvalidChar(val as u128)),
2019-07-26T13:58:45.9716208Z     | 
2019-07-26T13:58:45.9716569Z    ::: src/librustc/mir/interpret/error.rs:404:1
2019-07-26T13:58:45.9716757Z     |
2019-07-26T13:58:45.9716963Z 404 | pub enum InterpError<'tcx> {
2019-07-26T13:58:45.9716963Z 404 | pub enum InterpError<'tcx> {
2019-07-26T13:58:45.9717215Z     | -------------------------- variant or associated item `InvalidChar` not found here
2019-07-26T13:58:45.9717267Z 
2019-07-26T13:58:46.0033388Z error[E0599]: no variant or associated item named `ReadUndefBytes` found for type `mir::interpret::error::InterpError<'_>` in the current scope
2019-07-26T13:58:46.0033746Z    --> src/librustc/mir/interpret/value.rs:540:45
2019-07-26T13:58:46.0034027Z     |
2019-07-26T13:58:46.0034362Z 540 |             ScalarMaybeUndef::Undef => err!(ReadUndefBytes(Size::from_bytes(0))),
2019-07-26T13:58:46.0035059Z     | 
2019-07-26T13:58:46.0035328Z    ::: src/librustc/mir/interpret/error.rs:404:1
2019-07-26T13:58:46.0035576Z     |
2019-07-26T13:58:46.0035854Z 404 | pub enum InterpError<'tcx> {
2019-07-26T13:58:46.0035854Z 404 | pub enum InterpError<'tcx> {
2019-07-26T13:58:46.0036738Z     | -------------------------- variant or associated item `ReadUndefBytes` not found here
2019-07-26T13:58:46.0036802Z 
2019-07-26T13:58:46.0868491Z error[E0599]: no variant or associated item named `UnterminatedCString` found for type `mir::interpret::error::InterpError<'_>` in the current scope
2019-07-26T13:58:46.0869173Z    --> src/librustc/mir/interpret/allocation.rs:247:26
2019-07-26T13:58:46.0869396Z     |
2019-07-26T13:58:46.0869692Z 247 |             None => err!(UnterminatedCString(ptr.erase_tag())),
2019-07-26T13:58:46.0870536Z     | 
2019-07-26T13:58:46.0870977Z    ::: src/librustc/mir/interpret/error.rs:404:1
2019-07-26T13:58:46.0871248Z     |
2019-07-26T13:58:46.0871524Z 404 | pub enum InterpError<'tcx> {
2019-07-26T13:58:46.0871524Z 404 | pub enum InterpError<'tcx> {
2019-07-26T13:58:46.0872079Z     | -------------------------- variant or associated item `UnterminatedCString` not found here
2019-07-26T13:58:46.0872143Z 
2019-07-26T13:58:46.1067875Z error[E0599]: no variant or associated item named `ReadPointerAsBytes` found for type `mir::interpret::error::InterpError<'_>` in the current scope
2019-07-26T13:58:46.1068212Z    --> src/librustc/mir/interpret/allocation.rs:449:18
2019-07-26T13:58:46.1068402Z     |
2019-07-26T13:58:46.1068641Z 449 |             err!(ReadPointerAsBytes)
2019-07-26T13:58:46.1069404Z     | 
2019-07-26T13:58:46.1069634Z    ::: src/librustc/mir/interpret/error.rs:404:1
2019-07-26T13:58:46.1069841Z     |
2019-07-26T13:58:46.1070069Z 404 | pub enum InterpError<'tcx> {
2019-07-26T13:58:46.1070069Z 404 | pub enum InterpError<'tcx> {
2019-07-26T13:58:46.1070547Z     | -------------------------- variant or associated item `ReadPointerAsBytes` not found here
2019-07-26T13:58:46.1070609Z 
2019-07-26T13:58:46.1996329Z error: no variant `PointerOutOfBounds` in enum `mir::interpret::error::InterpError<'_>`
2019-07-26T13:58:46.1996643Z    --> src/librustc/mir/interpret/pointer.rs:201:18
2019-07-26T13:58:46.1996902Z     |
2019-07-26T13:58:46.1997128Z 201 |             err!(PointerOutOfBounds {
2019-07-26T13:58:46.1997561Z     | 
2019-07-26T13:58:46.1997767Z    ::: src/librustc/mir/interpret/error.rs:404:1
2019-07-26T13:58:46.1998174Z     |
2019-07-26T13:58:46.1998415Z 404 | pub enum InterpError<'tcx> {
2019-07-26T13:58:46.1998415Z 404 | pub enum InterpError<'tcx> {
2019-07-26T13:58:46.1998693Z     | -------------------------- variant `PointerOutOfBounds` not found here
2019-07-26T13:58:46.1999304Z    ::: src/librustc/mir/interpret/mod.rs:5:27
2019-07-26T13:58:46.1999487Z     |
2019-07-26T13:58:46.1999487Z     |
2019-07-26T13:58:46.2000019Z 5   |     ($($tt:tt)*) => { Err($crate::mir::interpret::InterpError::$($tt)*.into()) };
2019-07-26T13:58:46.2000533Z     |                           ------ variant not found in `mir::interpret::error::InterpError<'_>`
2019-07-26T13:58:54.8649365Z error: aborting due to 34 previous errors
2019-07-26T13:58:54.8649567Z 
2019-07-26T13:58:54.8649840Z Some errors have detailed explanations: E0422, E0599.
2019-07-26T13:58:54.8650124Z For more information about an error, try `rustc --explain E0422`.
2019-07-26T13:58:54.8650124Z For more information about an error, try `rustc --explain E0422`.
2019-07-26T13:58:55.0530384Z error: Could not compile `rustc`.
2019-07-26T13:58:55.0531230Z 
2019-07-26T13:58:55.0531901Z To learn more, run the command again with --verbose.
2019-07-26T13:58:55.0553406Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-26T13:58:55.0563554Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-26T13:58:55.0563815Z Build completed unsuccessfully in 0:05:02
2019-07-26T13:58:55.0563815Z Build completed unsuccessfully in 0:05:02
2019-07-26T13:58:55.8309038Z ##[error]Bash exited with code '1'.
2019-07-26T13:58:55.8339835Z ##[section]Starting: Checkout
2019-07-26T13:58:55.8341348Z ==============================================================================
2019-07-26T13:58:55.8341418Z Task         : Get sources
2019-07-26T13:58:55.8341462Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
