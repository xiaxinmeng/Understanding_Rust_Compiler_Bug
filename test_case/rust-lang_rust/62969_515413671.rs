plain
2019-07-26T11:02:00.9079480Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-26T11:02:00.9256406Z ##[command]git config gc.auto 0
2019-07-26T11:02:00.9334999Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-26T11:02:00.9374761Z ##[command]git config --get-all http.proxy
2019-07-26T11:02:00.9517740Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62969/merge:refs/remotes/pull/62969/merge
---
2019-07-26T11:02:39.7073270Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-26T11:02:39.7073305Z 
2019-07-26T11:02:39.7073559Z   git checkout -b <new-branch-name>
2019-07-26T11:02:39.7073593Z 
2019-07-26T11:02:39.7073649Z HEAD is now at 8b4a803fa Merge b7718ee6cc37dad7c7950dd3fcad0b832cfaa16e into 4268e7ee22935f086b856ef0063a9e22b49aeddb
2019-07-26T11:02:39.7229044Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-26T11:02:39.7232272Z ==============================================================================
2019-07-26T11:02:39.7232335Z Task         : Bash
2019-07-26T11:02:39.7232383Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-26T11:09:49.7249601Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-07-26T11:09:51.1893795Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-07-26T11:09:52.3962635Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-07-26T11:10:05.7563695Z     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
2019-07-26T11:10:10.5318255Z error[E0422]: cannot find struct, variant or union type `PointerOutOfBounds` in this scope
2019-07-26T11:10:10.5319956Z     |
2019-07-26T11:10:10.5319956Z     |
2019-07-26T11:10:10.5320528Z 434 |             PointerOutOfBounds { ptr, msg, allocation_size } => {
2019-07-26T11:10:10.5322095Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T11:10:10.5322573Z     |
2019-07-26T11:10:10.5322573Z     |
2019-07-26T11:10:10.5323159Z 1   | use crate::mir::interpret::error::UnsupportedInfo::PointerOutOfBounds;
2019-07-26T11:10:10.5324285Z 
2019-07-26T11:10:10.5324285Z 
2019-07-26T11:10:10.5461497Z error[E0531]: cannot find tuple struct/variant `ValidationFailure` in this scope
2019-07-26T11:10:10.5462420Z     |
2019-07-26T11:10:10.5462759Z 439 |             ValidationFailure(ref err) => {
2019-07-26T11:10:10.5463135Z     |             ^^^^^^^^^^^^^^^^^ not found in this scope
2019-07-26T11:10:10.5463498Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T11:10:10.5463498Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T11:10:10.5463762Z     |
2019-07-26T11:10:10.5464667Z 1   | use crate::mir::interpret::error::UnsupportedInfo::ValidationFailure;
2019-07-26T11:10:10.5465008Z 
2019-07-26T11:10:10.5465008Z 
2019-07-26T11:10:10.5596011Z error[E0531]: cannot find tuple struct/variant `NoMirFor` in this scope
2019-07-26T11:10:10.5596773Z     |
2019-07-26T11:10:10.5596773Z     |
2019-07-26T11:10:10.5597130Z 442 |             NoMirFor(ref func) => write!(f, "no mir for `{}`", func),
2019-07-26T11:10:10.5597916Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T11:10:10.5598208Z     |
2019-07-26T11:10:10.5598208Z     |
2019-07-26T11:10:10.5598580Z 1   | use crate::mir::interpret::error::UnsupportedInfo::NoMirFor;
2019-07-26T11:10:10.5598935Z 
2019-07-26T11:10:10.5598935Z 
2019-07-26T11:10:10.5737980Z error[E0531]: cannot find tuple struct/variant `FunctionAbiMismatch` in this scope
2019-07-26T11:10:10.5738724Z     |
2019-07-26T11:10:10.5738724Z     |
2019-07-26T11:10:10.5739060Z 443 |             FunctionAbiMismatch(caller_abi, callee_abi) =>
2019-07-26T11:10:10.5739784Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T11:10:10.5740027Z     |
2019-07-26T11:10:10.5740027Z     |
2019-07-26T11:10:10.5740413Z 1   | use crate::mir::interpret::error::UnsupportedInfo::FunctionAbiMismatch;
2019-07-26T11:10:10.5740709Z 
2019-07-26T11:10:10.5740709Z 
2019-07-26T11:10:10.5880667Z error[E0531]: cannot find tuple struct/variant `FunctionArgMismatch` in this scope
2019-07-26T11:10:10.5881335Z     |
2019-07-26T11:10:10.5881335Z     |
2019-07-26T11:10:10.5881721Z 446 |             FunctionArgMismatch(caller_ty, callee_ty) =>
2019-07-26T11:10:10.5882445Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T11:10:10.5882984Z     |
2019-07-26T11:10:10.5882984Z     |
2019-07-26T11:10:10.5883430Z 1   | use crate::mir::interpret::error::UnsupportedInfo::FunctionArgMismatch;
2019-07-26T11:10:10.5883790Z 
2019-07-26T11:10:10.5883790Z 
2019-07-26T11:10:10.6017200Z error[E0531]: cannot find tuple struct/variant `FunctionRetMismatch` in this scope
2019-07-26T11:10:10.6018076Z     |
2019-07-26T11:10:10.6018076Z     |
2019-07-26T11:10:10.6018408Z 450 |             FunctionRetMismatch(caller_ty, callee_ty) =>
2019-07-26T11:10:10.6019212Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T11:10:10.6019510Z     |
2019-07-26T11:10:10.6019510Z     |
2019-07-26T11:10:10.6019911Z 1   | use crate::mir::interpret::error::UnsupportedInfo::FunctionRetMismatch;
2019-07-26T11:10:10.6020334Z 
2019-07-26T11:10:10.6020334Z 
2019-07-26T11:10:10.6158833Z error[E0531]: cannot find tuple struct/variant `ReallocatedWrongMemoryKind` in this scope
2019-07-26T11:10:10.6159472Z     |
2019-07-26T11:10:10.6159472Z     |
2019-07-26T11:10:10.6159836Z 456 |             ReallocatedWrongMemoryKind(ref old, ref new) =>
2019-07-26T11:10:10.6160607Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T11:10:10.6160883Z     |
2019-07-26T11:10:10.6160883Z     |
2019-07-26T11:10:10.6161232Z 1   | use crate::mir::interpret::error::UnsupportedInfo::ReallocatedWrongMemoryKind;
2019-07-26T11:10:10.6161536Z 
2019-07-26T11:10:10.6161536Z 
2019-07-26T11:10:10.6315440Z error[E0531]: cannot find tuple struct/variant `DeallocatedWrongMemoryKind` in this scope
2019-07-26T11:10:10.6316177Z     |
2019-07-26T11:10:10.6316177Z     |
2019-07-26T11:10:10.6317231Z 458 |             DeallocatedWrongMemoryKind(ref old, ref new) =>
2019-07-26T11:10:10.6318058Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T11:10:10.6318588Z     |
2019-07-26T11:10:10.6318588Z     |
2019-07-26T11:10:10.6319010Z 1   | use crate::mir::interpret::error::UnsupportedInfo::DeallocatedWrongMemoryKind;
2019-07-26T11:10:10.6319328Z 
2019-07-26T11:10:10.6319328Z 
2019-07-26T11:10:10.6455665Z error[E0531]: cannot find tuple struct/variant `InvalidChar` in this scope
2019-07-26T11:10:10.6456329Z     |
2019-07-26T11:10:10.6456329Z     |
2019-07-26T11:10:10.6457128Z 460 |             InvalidChar(c) =>
2019-07-26T11:10:10.6458028Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T11:10:10.6458387Z     |
2019-07-26T11:10:10.6458387Z     |
2019-07-26T11:10:10.6458758Z 1   | use crate::mir::interpret::error::UnsupportedInfo::InvalidChar;
2019-07-26T11:10:10.6459086Z 
2019-07-26T11:10:10.6459086Z 
2019-07-26T11:10:10.6598710Z error[E0422]: cannot find struct, variant or union type `AlignmentCheckFailed` in this scope
2019-07-26T11:10:10.6600021Z     |
2019-07-26T11:10:10.6600021Z     |
2019-07-26T11:10:10.6600588Z 462 |             AlignmentCheckFailed { required, has } =>
2019-07-26T11:10:10.6601773Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T11:10:10.6602300Z     |
2019-07-26T11:10:10.6602300Z     |
2019-07-26T11:10:10.6602878Z 1   | use crate::mir::interpret::error::UnsupportedInfo::AlignmentCheckFailed;
2019-07-26T11:10:10.6603563Z 
2019-07-26T11:10:10.6603563Z 
2019-07-26T11:10:10.6739420Z error[E0531]: cannot find tuple struct/variant `TypeNotPrimitive` in this scope
2019-07-26T11:10:10.6740880Z     |
2019-07-26T11:10:10.6740880Z     |
2019-07-26T11:10:10.6741397Z 465 |             TypeNotPrimitive(ty) =>
2019-07-26T11:10:10.6742613Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T11:10:10.6743049Z     |
2019-07-26T11:10:10.6743049Z     |
2019-07-26T11:10:10.6743607Z 1   | use crate::mir::interpret::error::UnsupportedInfo::TypeNotPrimitive;
2019-07-26T11:10:10.6745031Z 
2019-07-26T11:10:10.6745031Z 
2019-07-26T11:10:10.6891382Z error[E0531]: cannot find tuple struct/variant `Layout` in this scope
2019-07-26T11:10:10.6892902Z     |
2019-07-26T11:10:10.6893444Z 467 |             Layout(ref err) =>
2019-07-26T11:10:10.6894636Z     |             ^^^^^^ not found in this scope
2019-07-26T11:10:10.6895303Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T11:10:10.6895303Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T11:10:10.6895767Z     |
2019-07-26T11:10:10.6896319Z 1   | use crate::mir::interpret::error::InvalidProgramInfo::Layout;
2019-07-26T11:10:10.6897172Z 
2019-07-26T11:10:10.6897172Z 
2019-07-26T11:10:10.7039448Z error[E0531]: cannot find tuple struct/variant `PathNotFound` in this scope
2019-07-26T11:10:10.7045020Z     |
2019-07-26T11:10:10.7045020Z     |
2019-07-26T11:10:10.7045475Z 469 |             PathNotFound(ref path) =>
2019-07-26T11:10:10.7046186Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T11:10:10.7047361Z     |
2019-07-26T11:10:10.7047361Z     |
2019-07-26T11:10:10.7047817Z 1   | use crate::mir::interpret::error::UnsupportedInfo::PathNotFound;
2019-07-26T11:10:10.7048152Z 
2019-07-26T11:10:10.7048152Z 
2019-07-26T11:10:10.7185578Z error[E0531]: cannot find tuple struct/variant `IncorrectAllocationInformation` in this scope
2019-07-26T11:10:10.7186371Z     |
2019-07-26T11:10:10.7186371Z     |
2019-07-26T11:10:10.7186738Z 471 |             IncorrectAllocationInformation(size, size2, align, align2) =>
2019-07-26T11:10:10.7187503Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T11:10:10.7187831Z     |
2019-07-26T11:10:10.7187831Z     |
2019-07-26T11:10:10.7188231Z 1   | use crate::mir::interpret::error::UnsupportedInfo::IncorrectAllocationInformation;
2019-07-26T11:10:10.7188632Z 
2019-07-26T11:10:10.7188632Z 
2019-07-26T11:10:10.7399807Z error[E0531]: cannot find tuple struct/variant `InvalidDiscriminant` in this scope
2019-07-26T11:10:10.7400612Z     |
2019-07-26T11:10:10.7400612Z     |
2019-07-26T11:10:10.7400949Z 475 |             InvalidDiscriminant(val) =>
2019-07-26T11:10:10.7401687Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T11:10:10.7401993Z     |
2019-07-26T11:10:10.7401993Z     |
2019-07-26T11:10:10.7403599Z 1   | use crate::mir::interpret::error::UnsupportedInfo::InvalidDiscriminant;
2019-07-26T11:10:10.7403956Z 
2019-07-26T11:10:10.7403956Z 
2019-07-26T11:10:10.7557968Z error[E0531]: cannot find tuple struct/variant `Exit` in this scope
2019-07-26T11:10:10.7590378Z     |
2019-07-26T11:10:10.7590378Z     |
2019-07-26T11:10:10.7590715Z 477 |             Exit(code) =>
2019-07-26T11:10:10.7591451Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T11:10:10.7591711Z     |
2019-07-26T11:10:10.7591711Z     |
2019-07-26T11:10:10.7592408Z 1   | use crate::mir::interpret::error::UnsupportedInfo::Exit;
2019-07-26T11:10:10.7592785Z 
2019-07-26T11:10:10.7592785Z 
2019-07-26T11:10:10.7714722Z error[E0531]: cannot find tuple struct/variant `InvalidBoolOp` in this scope
2019-07-26T11:10:10.7715414Z     |
2019-07-26T11:10:10.7715414Z     |
2019-07-26T11:10:10.7715753Z 549 |             InvalidBoolOp(_) =>
2019-07-26T11:10:10.7716465Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T11:10:10.7716769Z     |
2019-07-26T11:10:10.7716769Z     |
2019-07-26T11:10:10.7717133Z 1   | use crate::mir::interpret::error::UnsupportedInfo::InvalidBoolOp;
2019-07-26T11:10:10.7717517Z 
2019-07-26T11:10:10.7717517Z 
2019-07-26T11:10:10.7873282Z error[E0531]: cannot find tuple struct/variant `UnterminatedCString` in this scope
2019-07-26T11:10:10.7874066Z     |
2019-07-26T11:10:10.7874066Z     |
2019-07-26T11:10:10.7874421Z 551 |             UnterminatedCString(_) =>
2019-07-26T11:10:10.7883758Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T11:10:10.7884222Z     |
2019-07-26T11:10:10.7884222Z     |
2019-07-26T11:10:10.7884598Z 1   | use crate::mir::interpret::error::UnsupportedInfo::UnterminatedCString;
2019-07-26T11:10:10.7884951Z 
2019-07-26T11:10:10.7884951Z 
2019-07-26T11:10:10.8043737Z error[E0531]: cannot find tuple struct/variant `ReadUndefBytes` in this scope
2019-07-26T11:10:10.8044573Z     |
2019-07-26T11:10:10.8044573Z     |
2019-07-26T11:10:10.8044894Z 554 |             ReadUndefBytes(_) =>
2019-07-26T11:10:10.8045668Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T11:10:10.8045936Z     |
2019-07-26T11:10:10.8045936Z     |
2019-07-26T11:10:10.8046309Z 1   | use crate::mir::interpret::error::UnsupportedInfo::ReadUndefBytes;
2019-07-26T11:10:10.8046871Z 
2019-07-26T11:10:10.8046871Z 
2019-07-26T11:10:10.8190138Z error[E0531]: cannot find tuple struct/variant `HeapAllocNonPowerOfTwoAlignment` in this scope
2019-07-26T11:10:10.8190826Z     |
2019-07-26T11:10:10.8190826Z     |
2019-07-26T11:10:10.8191189Z 556 |             HeapAllocNonPowerOfTwoAlignment(_) =>
2019-07-26T11:10:10.8192322Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T11:10:10.8192666Z     |
2019-07-26T11:10:10.8192666Z     |
2019-07-26T11:10:10.8193033Z 1   | use crate::mir::interpret::error::UnsupportedInfo::HeapAllocNonPowerOfTwoAlignment;
2019-07-26T11:10:10.8193345Z 
2019-07-26T11:10:10.8193345Z 
2019-07-26T11:10:10.8356216Z error[E0531]: cannot find tuple struct/variant `MachineError` in this scope
2019-07-26T11:10:10.8356915Z     |
2019-07-26T11:10:10.8356915Z     |
2019-07-26T11:10:10.8357207Z 559 |             MachineError(ref msg) |
2019-07-26T11:10:10.8357912Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T11:10:10.8358173Z     |
2019-07-26T11:10:10.8358173Z     |
2019-07-26T11:10:10.8358489Z 1   | use crate::mir::interpret::error::UnsupportedInfo::MachineError;
2019-07-26T11:10:10.8358768Z 
2019-07-26T11:10:10.8358768Z 
2019-07-26T11:10:10.8502672Z error[E0531]: cannot find tuple struct/variant `Unimplemented` in this scope
2019-07-26T11:10:10.8503432Z     |
2019-07-26T11:10:10.8503432Z     |
2019-07-26T11:10:10.8503761Z 560 |             Unimplemented(ref msg) |
2019-07-26T11:10:10.8505011Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T11:10:10.8505282Z     |
2019-07-26T11:10:10.8505282Z     |
2019-07-26T11:10:10.8505658Z 1   | use crate::mir::interpret::error::UnsupportedInfo::Unimplemented;
2019-07-26T11:10:10.8505944Z 
2019-07-26T11:10:10.8505944Z 
2019-07-26T11:10:10.8653044Z error[E0531]: cannot find tuple struct/variant `AbiViolation` in this scope
2019-07-26T11:10:10.8653680Z     |
2019-07-26T11:10:10.8653680Z     |
2019-07-26T11:10:10.8654401Z 561 |             AbiViolation(ref msg) |
2019-07-26T11:10:10.8655316Z help: possible candidate is found in another module, you can import it into scope
2019-07-26T11:10:10.8655617Z     |
2019-07-26T11:10:10.8655617Z     |
2019-07-26T11:10:10.8655960Z 1   | use crate::mir::interpret::error::UnsupportedInfo::AbiViolation;
2019-07-26T11:10:10.8656265Z 
2019-07-26T11:10:10.8656265Z 
2019-07-26T11:10:10.8818679Z error[E0531]: cannot find tuple struct/variant `Intrinsic` in this scope
2019-07-26T11:10:10.8819565Z     |
2019-07-26T11:10:10.8819565Z     |
2019-07-26T11:10:10.8819898Z 562 |             Intrinsic(ref msg) =>
2019-07-26T11:10:10.8820568Z help: possible candidates are found in other modules, you can import them into scope
2019-07-26T11:10:10.8820782Z     |
2019-07-26T11:10:10.8820782Z     |
2019-07-26T11:10:10.8821072Z 1   | use crate::mir::interpret::error::UnsupportedInfo::Intrinsic;
2019-07-26T11:10:10.8821303Z     |
2019-07-26T11:10:10.8821574Z 1   | use crate::ty::instance::InstanceDef::Intrinsic;
2019-07-26T11:10:10.8821925Z 
2019-07-26T11:10:10.8821925Z 
2019-07-26T11:10:23.3172842Z error[E0599]: no variant or associated item named `ReadUndefBytes` found for type `mir::interpret::error::InterpError<'_>` in the current scope
2019-07-26T11:10:23.3174128Z    --> src/librustc/mir/interpret/allocation.rs:519:30
2019-07-26T11:10:23.3174771Z     |
2019-07-26T11:10:23.3175492Z 519 |         ).or_else(|idx| err!(ReadUndefBytes(idx)))
2019-07-26T11:10:23.3179854Z     | 
2019-07-26T11:10:23.3180354Z    ::: src/librustc/mir/interpret/error.rs:406:1
2019-07-26T11:10:23.3180624Z     |
2019-07-26T11:10:23.3180972Z 406 | pub enum InterpError<'tcx> {
2019-07-26T11:10:23.3180972Z 406 | pub enum InterpError<'tcx> {
2019-07-26T11:10:23.3181372Z     | -------------------------- variant or associated item `ReadUndefBytes` not found here
2019-07-26T11:10:23.3181436Z 
2019-07-26T11:10:23.4155044Z error[E0599]: no variant or associated item named `Layout` found for type `mir::interpret::error::InterpError<'_>` in the current scope
2019-07-26T11:10:23.4156599Z     |
2019-07-26T11:10:23.4156599Z     |
2019-07-26T11:10:23.4157151Z 145 |             InterpError::Layout(LayoutError::SizeOverflow(_)) |
2019-07-26T11:10:23.4158311Z ...
2019-07-26T11:10:23.4158951Z 406 | pub enum InterpError<'tcx> {
2019-07-26T11:10:23.4158951Z 406 | pub enum InterpError<'tcx> {
2019-07-26T11:10:23.4159834Z     | -------------------------- variant or associated item `Layout` not found here
2019-07-26T11:10:23.4160060Z 
2019-07-26T11:10:24.5175927Z error[E0599]: no variant or associated item named `ReadPointerAsBytes` found for type `mir::interpret::error::InterpError<'_>` in the current scope
2019-07-26T11:10:24.5187415Z    --> src/librustc/mir/interpret/value.rs:363:36
2019-07-26T11:10:24.5188096Z     |
2019-07-26T11:10:24.5189495Z 363 |             Scalar::Ptr(_) => err!(ReadPointerAsBytes),
2019-07-26T11:10:24.5191138Z     | 
2019-07-26T11:10:24.5191767Z    ::: src/librustc/mir/interpret/error.rs:406:1
2019-07-26T11:10:24.5192333Z     |
2019-07-26T11:10:24.5193127Z 406 | pub enum InterpError<'tcx> {
2019-07-26T11:10:24.5193127Z 406 | pub enum InterpError<'tcx> {
2019-07-26T11:10:24.5193935Z     | -------------------------- variant or associated item `ReadPointerAsBytes` not found here
2019-07-26T11:10:24.5194143Z 
2019-07-26T11:10:24.5194959Z error[E0599]: no variant or associated item named `InvalidNullPointerUsage` found for type `mir::interpret::error::InterpError<'_>` in the current scope
2019-07-26T11:10:24.5195527Z    --> src/librustc/mir/interpret/value.rs:376:49
2019-07-26T11:10:24.5196003Z     |
2019-07-26T11:10:24.5196542Z 376 |             Scalar::Raw { data: 0, .. } => err!(InvalidNullPointerUsage),
2019-07-26T11:10:24.5197733Z     | 
2019-07-26T11:10:24.5198212Z    ::: src/librustc/mir/interpret/error.rs:406:1
2019-07-26T11:10:24.5198661Z     |
2019-07-26T11:10:24.5199661Z 406 | pub enum InterpError<'tcx> {
2019-07-26T11:10:24.5199661Z 406 | pub enum InterpError<'tcx> {
2019-07-26T11:10:24.5200312Z     | -------------------------- variant or associated item `InvalidNullPointerUsage` not found here
2019-07-26T11:10:24.5200529Z 
2019-07-26T11:10:24.5201091Z error[E0599]: no variant or associated item named `ReadBytesAsPointer` found for type `mir::interpret::error::InterpError<'_>` in the current scope
2019-07-26T11:10:24.5201616Z    --> src/librustc/mir/interpret/value.rs:377:40
2019-07-26T11:10:24.5202089Z     |
2019-07-26T11:10:24.5202608Z 377 |             Scalar::Raw { .. } => err!(ReadBytesAsPointer),
2019-07-26T11:10:24.5203742Z     | 
2019-07-26T11:10:24.5204231Z    ::: src/librustc/mir/interpret/error.rs:406:1
2019-07-26T11:10:24.5204900Z     |
2019-07-26T11:10:24.5205392Z 406 | pub enum InterpError<'tcx> {
2019-07-26T11:10:24.5205392Z 406 | pub enum InterpError<'tcx> {
2019-07-26T11:10:24.5205980Z     | -------------------------- variant or associated item `ReadBytesAsPointer` not found here
2019-07-26T11:10:24.5206171Z 
2019-07-26T11:10:24.5206749Z error[E0599]: no variant or associated item named `InvalidBool` found for type `mir::interpret::error::InterpError<'_>` in the current scope
2019-07-26T11:10:24.5207243Z    --> src/librustc/mir/interpret/value.rs:409:23
2019-07-26T11:10:24.5207806Z     |
2019-07-26T11:10:24.5208404Z 409 |             _ => err!(InvalidBool),
2019-07-26T11:10:24.5209944Z     | 
2019-07-26T11:10:24.5210422Z    ::: src/librustc/mir/interpret/error.rs:406:1
2019-07-26T11:10:24.5210856Z     |
2019-07-26T11:10:24.5211363Z 406 | pub enum InterpError<'tcx> {
2019-07-26T11:10:24.5211363Z 406 | pub enum InterpError<'tcx> {
2019-07-26T11:10:24.5211940Z     | -------------------------- variant or associated item `InvalidBool` not found here
2019-07-26T11:10:24.5212187Z 
2019-07-26T11:10:24.5212729Z error[E0599]: no variant or associated item named `InvalidChar` found for type `mir::interpret::error::InterpError<'_>` in the current scope
2019-07-26T11:10:24.5213268Z    --> src/librustc/mir/interpret/value.rs:417:26
2019-07-26T11:10:24.5213712Z     |
2019-07-26T11:10:24.5214222Z 417 |             None => err!(InvalidChar(val as u128)),
2019-07-26T11:10:24.5215368Z     | 
2019-07-26T11:10:24.5215856Z    ::: src/librustc/mir/interpret/error.rs:406:1
2019-07-26T11:10:24.5216305Z     |
2019-07-26T11:10:24.5216925Z 406 | pub enum InterpError<'tcx> {
2019-07-26T11:10:24.5216925Z 406 | pub enum InterpError<'tcx> {
2019-07-26T11:10:24.5217523Z     | -------------------------- variant or associated item `InvalidChar` not found here
2019-07-26T11:10:24.5217723Z 
2019-07-26T11:10:24.5218273Z error[E0599]: no variant or associated item named `ReadUndefBytes` found for type `mir::interpret::error::InterpError<'_>` in the current scope
2019-07-26T11:10:24.5221810Z    --> src/librustc/mir/interpret/value.rs:540:45
2019-07-26T11:10:24.5222296Z     |
2019-07-26T11:10:24.5222663Z 540 |             ScalarMaybeUndef::Undef => err!(ReadUndefBytes(Size::from_bytes(0))),
2019-07-26T11:10:24.5223491Z     | 
2019-07-26T11:10:24.5223978Z    ::: src/librustc/mir/interpret/error.rs:406:1
2019-07-26T11:10:24.5225200Z     |
2019-07-26T11:10:24.5225676Z 406 | pub enum InterpError<'tcx> {
2019-07-26T11:10:24.5225676Z 406 | pub enum InterpError<'tcx> {
2019-07-26T11:10:24.5226076Z     | -------------------------- variant or associated item `ReadUndefBytes` not found here
2019-07-26T11:10:24.5226150Z 
2019-07-26T11:10:24.5226544Z error[E0599]: no variant or associated item named `UnterminatedCString` found for type `mir::interpret::error::InterpError<'_>` in the current scope
2019-07-26T11:10:24.5227005Z    --> src/librustc/mir/interpret/allocation.rs:247:26
2019-07-26T11:10:24.5227316Z     |
2019-07-26T11:10:24.5227652Z 247 |             None => err!(UnterminatedCString(ptr.erase_tag())),
2019-07-26T11:10:24.5229002Z     | 
2019-07-26T11:10:24.5229384Z    ::: src/librustc/mir/interpret/error.rs:406:1
2019-07-26T11:10:24.5229655Z     |
2019-07-26T11:10:24.5229965Z 406 | pub enum InterpError<'tcx> {
2019-07-26T11:10:24.5229965Z 406 | pub enum InterpError<'tcx> {
2019-07-26T11:10:24.5230375Z     | -------------------------- variant or associated item `UnterminatedCString` not found here
2019-07-26T11:10:24.5230448Z 
2019-07-26T11:10:24.5230847Z error[E0599]: no variant or associated item named `ReadPointerAsBytes` found for type `mir::interpret::error::InterpError<'_>` in the current scope
2019-07-26T11:10:24.5231174Z    --> src/librustc/mir/interpret/allocation.rs:449:18
2019-07-26T11:10:24.5231434Z     |
2019-07-26T11:10:24.5231742Z 449 |             err!(ReadPointerAsBytes)
2019-07-26T11:10:24.5232446Z     | 
2019-07-26T11:10:24.5232745Z    ::: src/librustc/mir/interpret/error.rs:406:1
2019-07-26T11:10:24.5232994Z     |
2019-07-26T11:10:24.5233311Z 406 | pub enum InterpError<'tcx> {
2019-07-26T11:10:24.5233311Z 406 | pub enum InterpError<'tcx> {
2019-07-26T11:10:24.5233722Z     | -------------------------- variant or associated item `ReadPointerAsBytes` not found here
2019-07-26T11:10:24.5233770Z 
2019-07-26T11:10:24.5234110Z error: no variant `PointerOutOfBounds` in enum `mir::interpret::error::InterpError<'_>`
2019-07-26T11:10:24.5234411Z    --> src/librustc/mir/interpret/pointer.rs:201:18
2019-07-26T11:10:24.5234648Z     |
2019-07-26T11:10:24.5234972Z 201 |             err!(PointerOutOfBounds {
2019-07-26T11:10:24.5235543Z     | 
2019-07-26T11:10:24.5235851Z    ::: src/librustc/mir/interpret/error.rs:406:1
2019-07-26T11:10:24.5236266Z     |
2019-07-26T11:10:24.5236566Z 406 | pub enum InterpError<'tcx> {
2019-07-26T11:10:24.5236566Z 406 | pub enum InterpError<'tcx> {
2019-07-26T11:10:24.5236959Z     | -------------------------- variant `PointerOutOfBounds` not found here
2019-07-26T11:10:24.5237490Z    ::: src/librustc/mir/interpret/mod.rs:5:27
2019-07-26T11:10:24.5237749Z     |
2019-07-26T11:10:24.5237749Z     |
2019-07-26T11:10:24.5238102Z 5   |     ($($tt:tt)*) => { Err($crate::mir::interpret::InterpError::$($tt)*.into()) };
2019-07-26T11:10:24.5238992Z     |                           ------ variant not found in `mir::interpret::error::InterpError<'_>`
2019-07-26T11:10:32.1146391Z error: aborting due to 35 previous errors
2019-07-26T11:10:32.1147288Z 
2019-07-26T11:10:32.1147937Z Some errors have detailed explanations: E0422, E0599.
2019-07-26T11:10:32.1148591Z For more information about an error, try `rustc --explain E0422`.
2019-07-26T11:10:32.1148591Z For more information about an error, try `rustc --explain E0422`.
2019-07-26T11:10:32.3097850Z error: Could not compile `rustc`.
2019-07-26T11:10:32.3098596Z 
2019-07-26T11:10:32.3099365Z To learn more, run the command again with --verbose.
2019-07-26T11:10:32.3129467Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-26T11:10:32.3144223Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-26T11:10:32.3144801Z Build completed unsuccessfully in 0:05:03
2019-07-26T11:10:32.3144801Z Build completed unsuccessfully in 0:05:03
2019-07-26T11:10:33.0044506Z ##[error]Bash exited with code '1'.
2019-07-26T11:10:33.0084065Z ##[section]Starting: Checkout
2019-07-26T11:10:33.0085963Z ==============================================================================
2019-07-26T11:10:33.0086019Z Task         : Get sources
2019-07-26T11:10:33.0086064Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
