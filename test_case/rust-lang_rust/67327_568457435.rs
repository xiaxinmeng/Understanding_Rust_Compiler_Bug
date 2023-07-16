plain
2019-12-23T11:56:08.4725157Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-23T11:56:09.3018192Z ##[command]git config gc.auto 0
2019-12-23T11:56:09.3023726Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-23T11:56:09.3028281Z ##[command]git config --get-all http.proxy
2019-12-23T11:56:09.3030939Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67327/merge:refs/remotes/pull/67327/merge
---
2019-12-23T12:04:54.5552648Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-12-23T12:04:54.9096882Z error[E0252]: the name `ty` is defined multiple times
2019-12-23T12:04:54.9098717Z  --> src/librustc_mir/const_eval/eval_queries.rs:7:17
2019-12-23T12:04:54.9099320Z   |
2019-12-23T12:04:54.9100190Z 6 | use rustc::ty::{self, layout::LayoutOf, subst::Subst, TyCtxt};
2019-12-23T12:04:54.9100848Z   |                 ---- previous import of the module `ty` here
2019-12-23T12:04:54.9101295Z 7 | use rustc::ty::{self, TyCtxt};
2019-12-23T12:04:54.9102253Z   |                 |
2019-12-23T12:04:54.9102998Z   |                 `ty` reimported here
2019-12-23T12:04:54.9103388Z   |                 help: remove unnecessary import
2019-12-23T12:04:54.9103723Z   |
2019-12-23T12:04:54.9103723Z   |
2019-12-23T12:04:54.9104115Z   = note: `ty` must be defined only once in the type namespace of this module
2019-12-23T12:04:54.9123790Z 
2019-12-23T12:04:54.9212268Z error[E0252]: the name `TyCtxt` is defined multiple times
2019-12-23T12:04:54.9212637Z  --> src/librustc_mir/const_eval/eval_queries.rs:7:23
2019-12-23T12:04:54.9212925Z   |
2019-12-23T12:04:54.9213417Z 6 | use rustc::ty::{self, layout::LayoutOf, subst::Subst, TyCtxt};
2019-12-23T12:04:54.9213803Z   |                                                       ------ previous import of the type `TyCtxt` here
2019-12-23T12:04:54.9214104Z 7 | use rustc::ty::{self, TyCtxt};
2019-12-23T12:04:54.9214555Z   |                       ^^^^^^ `TyCtxt` reimported here
2019-12-23T12:04:54.9214785Z   |
2019-12-23T12:04:54.9215062Z   = note: `TyCtxt` must be defined only once in the type namespace of this module
2019-12-23T12:04:54.9218909Z 
2019-12-23T12:04:54.9490140Z error[E0255]: the name `mk_eval_cx` is defined multiple times
2019-12-23T12:04:54.9490443Z   --> src/librustc_mir/const_eval/eval_queries.rs:74:1
2019-12-23T12:04:54.9490671Z    |
2019-12-23T12:04:54.9491204Z 15 |       error_to_const_error, mk_eval_cx, op_to_const, CompileTimeEvalContext, CompileTimeInterpreter,
2019-12-23T12:04:54.9491602Z    |                             ---------- previous import of the value `mk_eval_cx` here
2019-12-23T12:04:54.9491959Z ...
2019-12-23T12:04:54.9492246Z 74 | / pub(super) fn mk_eval_cx<'mir, 'tcx>(
2019-12-23T12:04:54.9492523Z 75 | |     tcx: TyCtxt<'tcx>,
2019-12-23T12:04:54.9492811Z 76 | |     span: Span,
2019-12-23T12:04:54.9493105Z 77 | |     param_env: ty::ParamEnv<'tcx>,
2019-12-23T12:04:54.9493329Z ...  |
2019-12-23T12:04:54.9493691Z 80 | |     InterpCx::new(tcx.at(span), param_env, CompileTimeInterpreter::new(), Default::default())
2019-12-23T12:04:54.9493968Z 81 | | }
2019-12-23T12:04:54.9494261Z    | |_^ `mk_eval_cx` redefined here
2019-12-23T12:04:54.9494472Z    |
2019-12-23T12:04:54.9494766Z    = note: `mk_eval_cx` must be defined only once in the value namespace of this module
2019-12-23T12:04:54.9495251Z    |
2019-12-23T12:04:54.9495251Z    |
2019-12-23T12:04:54.9495587Z 15 |     error_to_const_error, mk_eval_cx as other_mk_eval_cx, op_to_const, CompileTimeEvalContext, CompileTimeInterpreter,
2019-12-23T12:04:54.9500810Z 
2019-12-23T12:04:54.9506938Z error[E0255]: the name `op_to_const` is defined multiple times
2019-12-23T12:04:54.9507275Z    --> src/librustc_mir/const_eval/eval_queries.rs:83:1
2019-12-23T12:04:54.9507505Z     |
2019-12-23T12:04:54.9507505Z     |
2019-12-23T12:04:54.9507896Z 15  |       error_to_const_error, mk_eval_cx, op_to_const, CompileTimeEvalContext, CompileTimeInterpreter,
2019-12-23T12:04:54.9508308Z     |                                         ----------- previous import of the value `op_to_const` here
2019-12-23T12:04:54.9508550Z ...
2019-12-23T12:04:54.9508867Z 83  | / pub(super) fn op_to_const<'tcx>(
2019-12-23T12:04:54.9509210Z 84  | |     ecx: &CompileTimeEvalContext<'_, 'tcx>,
2019-12-23T12:04:54.9509535Z 85  | |     op: OpTy<'tcx>,
2019-12-23T12:04:54.9510003Z 86  | | ) -> &'tcx ty::Const<'tcx> {
2019-12-23T12:04:54.9510236Z ...   |
2019-12-23T12:04:54.9510723Z 153 | |     ecx.tcx.mk_const(ty::Const { val: ty::ConstKind::Value(val), ty: op.layout.ty })
2019-12-23T12:04:54.9511039Z 154 | | }
2019-12-23T12:04:54.9511324Z     | |_^ `op_to_const` redefined here
2019-12-23T12:04:54.9511944Z     = note: `op_to_const` must be defined only once in the value namespace of this module
2019-12-23T12:04:54.9512209Z help: you can use `as` to change the binding name of the import
2019-12-23T12:04:54.9512435Z     |
2019-12-23T12:04:54.9512435Z     |
2019-12-23T12:04:54.9512780Z 15  |     error_to_const_error, mk_eval_cx, op_to_const as other_op_to_const, CompileTimeEvalContext, CompileTimeInterpreter,
2019-12-23T12:04:54.9564622Z 
2019-12-23T12:04:55.5975849Z error[E0433]: failed to resolve: use of undeclared type or module `layout`
2019-12-23T12:04:55.5976187Z   --> src/librustc_mir/const_eval/eval_queries.rs:95:9
2019-12-23T12:04:55.5977169Z    |
2019-12-23T12:04:55.5977169Z    |
2019-12-23T12:04:55.5977465Z 95 |         layout::Abi::Scalar(..) => true,
2019-12-23T12:04:55.5977802Z    |         ^^^^^^ use of undeclared type or module `layout`
2019-12-23T12:04:55.5982114Z 
2019-12-23T12:04:55.6008740Z error[E0433]: failed to resolve: use of undeclared type or module `layout`
2019-12-23T12:04:55.6009111Z   --> src/librustc_mir/const_eval/eval_queries.rs:96:9
2019-12-23T12:04:55.6009351Z    |
2019-12-23T12:04:55.6009667Z 96 |         layout::Abi::ScalarPair(..) => match op.layout.ty.kind {
2019-12-23T12:04:55.6014623Z 
2019-12-23T12:04:55.6087601Z error[E0433]: failed to resolve: use of undeclared type or module `Immediate`
2019-12-23T12:04:55.6087958Z    --> src/librustc_mir/const_eval/eval_queries.rs:123:26
2019-12-23T12:04:55.6088192Z     |
2019-12-23T12:04:55.6088192Z     |
2019-12-23T12:04:55.6088505Z 123 |         Err(ImmTy { imm: Immediate::Scalar(x), .. }) => match x {
2019-12-23T12:04:55.6088900Z     |                          ^^^^^^^^^ use of undeclared type or module `Immediate`
2019-12-23T12:04:55.6117352Z error[E0433]: failed to resolve: use of undeclared type or module `ScalarMaybeUndef`
2019-12-23T12:04:55.6117733Z    --> src/librustc_mir/const_eval/eval_queries.rs:124:13
2019-12-23T12:04:55.6117964Z     |
2019-12-23T12:04:55.6117964Z     |
2019-12-23T12:04:55.6118280Z 124 |             ScalarMaybeUndef::Scalar(s) => ConstValue::Scalar(s),
2019-12-23T12:04:55.6118667Z     |             ^^^^^^^^^^^^^^^^ use of undeclared type or module `ScalarMaybeUndef`
2019-12-23T12:04:55.6144933Z error[E0433]: failed to resolve: use of undeclared type or module `ScalarMaybeUndef`
2019-12-23T12:04:55.6145290Z    --> src/librustc_mir/const_eval/eval_queries.rs:125:13
2019-12-23T12:04:55.6145492Z     |
2019-12-23T12:04:55.6145740Z 125 |             ScalarMaybeUndef::Undef => {
2019-12-23T12:04:55.6145740Z 125 |             ScalarMaybeUndef::Undef => {
2019-12-23T12:04:55.6146194Z     |             ^^^^^^^^^^^^^^^^ use of undeclared type or module `ScalarMaybeUndef`
2019-12-23T12:04:55.6150054Z 
2019-12-23T12:04:55.6216896Z error[E0433]: failed to resolve: use of undeclared type or module `Immediate`
2019-12-23T12:04:55.6217294Z    --> src/librustc_mir/const_eval/eval_queries.rs:137:26
2019-12-23T12:04:55.6217527Z     |
2019-12-23T12:04:55.6217860Z 137 |         Err(ImmTy { imm: Immediate::ScalarPair(a, b), .. }) => {
2019-12-23T12:04:55.6218253Z     |                          ^^^^^^^^^ use of undeclared type or module `Immediate`
2019-12-23T12:04:55.6248077Z error[E0433]: failed to resolve: use of undeclared type or module `Scalar`
2019-12-23T12:04:55.6248429Z    --> src/librustc_mir/const_eval/eval_queries.rs:139:17
2019-12-23T12:04:55.6248655Z     |
2019-12-23T12:04:55.6248955Z 139 |                 Scalar::Ptr(ptr) => {
---
2019-12-23T12:04:55.6281881Z 
2019-12-23T12:04:55.6308691Z error[E0433]: failed to resolve: use of undeclared type or module `Allocation`
2019-12-23T12:04:55.6309020Z    --> src/librustc_mir/const_eval/eval_queries.rs:143:48
2019-12-23T12:04:55.6309249Z     |
2019-12-23T12:04:55.6309633Z 143 |                     ecx.tcx.intern_const_alloc(Allocation::from_byte_aligned_bytes(b"" as &[u8])),
2019-12-23T12:04:55.6314634Z 
2019-12-23T12:04:55.8426455Z error[E0412]: cannot find type `Span` in this scope
2019-12-23T12:04:55.8426887Z   --> src/librustc_mir/const_eval/eval_queries.rs:76:11
2019-12-23T12:04:55.8427209Z    |
---
2019-12-23T12:04:55.8508985Z 
2019-12-23T12:04:55.8565943Z error[E0422]: cannot find struct, variant or union type `ImmTy` in this scope
2019-12-23T12:04:55.8567816Z    --> src/librustc_mir/const_eval/eval_queries.rs:123:13
2019-12-23T12:04:55.8589173Z     |
2019-12-23T12:04:55.8589560Z 123 |         Err(ImmTy { imm: Immediate::Scalar(x), .. }) => match x {
2019-12-23T12:04:55.8590314Z     |
2019-12-23T12:04:55.8590765Z help: possible candidate is found in another module, you can import it into scope
2019-12-23T12:04:55.8591016Z     |
2019-12-23T12:04:55.8591303Z 1   | use crate::interpret::operand::ImmTy;
2019-12-23T12:04:55.8591303Z 1   | use crate::interpret::operand::ImmTy;
2019-12-23T12:04:55.8591522Z     |
2019-12-23T12:04:55.8591554Z 
2019-12-23T12:04:55.8624470Z error[E0422]: cannot find struct, variant or union type `ImmTy` in this scope
2019-12-23T12:04:55.8627658Z    --> src/librustc_mir/const_eval/eval_queries.rs:137:13
2019-12-23T12:04:55.8627901Z     |
2019-12-23T12:04:55.8628281Z 137 |         Err(ImmTy { imm: Immediate::ScalarPair(a, b), .. }) => {
2019-12-23T12:04:55.8628872Z     |
2019-12-23T12:04:55.8629168Z help: possible candidate is found in another module, you can import it into scope
2019-12-23T12:04:55.8629391Z     |
2019-12-23T12:04:55.8629693Z 1   | use crate::interpret::operand::ImmTy;
2019-12-23T12:04:55.8629693Z 1   | use crate::interpret::operand::ImmTy;
2019-12-23T12:04:55.8630235Z     |
2019-12-23T12:04:55.8630279Z 
2019-12-23T12:04:56.1302391Z error: unused imports: `TyCtxt`, `self`
2019-12-23T12:04:56.1302806Z  --> src/librustc_mir/const_eval/eval_queries.rs:7:17
2019-12-23T12:04:56.1303000Z   |
2019-12-23T12:04:56.1303553Z 7 | use rustc::ty::{self, TyCtxt};
2019-12-23T12:04:56.1304002Z   |
2019-12-23T12:04:56.1304255Z   = note: `-D unused-imports` implied by `-D warnings`
2019-12-23T12:04:56.1304289Z 
2019-12-23T12:04:56.1304289Z 
2019-12-23T12:04:56.1335724Z error: unused imports: `mk_eval_cx`, `op_to_const`
2019-12-23T12:04:56.1338159Z   --> src/librustc_mir/const_eval/eval_queries.rs:15:27
2019-12-23T12:04:56.1338411Z    |
2019-12-23T12:04:56.1338796Z 15 |     error_to_const_error, mk_eval_cx, op_to_const, CompileTimeEvalContext, CompileTimeInterpreter,
2019-12-23T12:04:56.1339216Z 
2019-12-23T12:05:06.2467344Z error: aborting due to 19 previous errors
2019-12-23T12:05:06.2468419Z 
2019-12-23T12:05:06.2477026Z Some errors have detailed explanations: E0252, E0255, E0412, E0422, E0433.
2019-12-23T12:05:06.2477026Z Some errors have detailed explanations: E0252, E0255, E0412, E0422, E0433.
2019-12-23T12:05:06.2484283Z For more information about an error, try `rustc --explain E0252`.
2019-12-23T12:05:06.2597898Z error: could not compile `rustc_mir`.
2019-12-23T12:05:06.2618482Z warning: build failed, waiting for other jobs to finish...
2019-12-23T12:05:08.3163031Z error: build failed
2019-12-23T12:05:08.3209507Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-23T12:05:08.3244044Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-12-23T12:05:08.3244382Z Build completed unsuccessfully in 0:05:42
2019-12-23T12:05:08.3272273Z == clock drift check ==
2019-12-23T12:05:08.3288406Z   local time: Mon Dec 23 12:05:08 UTC 2019
2019-12-23T12:05:08.3288406Z   local time: Mon Dec 23 12:05:08 UTC 2019
2019-12-23T12:05:08.8811055Z   network time: Mon, 23 Dec 2019 12:05:08 GMT
2019-12-23T12:05:08.8815807Z == end clock drift check ==
2019-12-23T12:05:10.0104040Z 
2019-12-23T12:05:10.0204596Z ##[error]Bash exited with code '1'.
2019-12-23T12:05:10.0236961Z ##[section]Starting: Checkout
2019-12-23T12:05:10.0238617Z ==============================================================================
2019-12-23T12:05:10.0238707Z Task         : Get sources
2019-12-23T12:05:10.0238755Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
