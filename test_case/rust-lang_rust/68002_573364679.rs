plain
2020-01-11T23:34:26.0429876Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-11T23:34:26.0445331Z ##[command]git config gc.auto 0
2020-01-11T23:34:26.0450771Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-11T23:34:26.0454660Z ##[command]git config --get-all http.proxy
2020-01-11T23:34:26.0462717Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68002/merge:refs/remotes/pull/68002/merge
---
2020-01-11T23:42:32.5082763Z     |
2020-01-11T23:42:32.5083425Z 292 | impl<'mir, 'tcx> ConstPropagator<'mir, 'tcx> {
2020-01-11T23:42:32.5083971Z     |                                              - un-closed delimiter
2020-01-11T23:42:32.5084186Z ...
2020-01-11T23:42:32.5084529Z 612 |                 if let PlaceRef { base: PlaceBase::Local(local), .. } = place_ref.as_ref() {
2020-01-11T23:42:32.5084965Z     |                                                                                            - this delimiter might not be properly closed...
2020-01-11T23:42:32.5085449Z 629 |             }
2020-01-11T23:42:32.5085968Z     |             - ...as it matches this but it has different indentation
2020-01-11T23:42:32.5086254Z ...
2020-01-11T23:42:32.5086495Z 976 | }
---
2020-01-11T23:42:32.5110202Z     |
2020-01-11T23:42:32.5110471Z 631 |             _ => {}
2020-01-11T23:42:32.5110783Z     |             ^ expected expression
2020-01-11T23:42:32.5110843Z 
2020-01-11T23:42:32.5128583Z error: expected one of `::`, `=>`, `if`, or `|`, found `.`
2020-01-11T23:42:32.5129153Z     |
2020-01-11T23:42:32.5129153Z     |
2020-01-11T23:42:32.5129443Z 634 |         self.use_ecx(source_info, |this| {
2020-01-11T23:42:32.5130023Z     |             ^ expected one of `::`, `=>`, `if`, or `|`
2020-01-11T23:42:32.5153178Z error: unexpected `self` parameter in function
2020-01-11T23:42:32.5153496Z    --> src/librustc_mir/transform/const_prop.rs:641:28
2020-01-11T23:42:32.5153744Z     |
2020-01-11T23:42:32.5153744Z     |
2020-01-11T23:42:32.5154106Z 641 |     fn operand_from_scalar(&self, scalar: Scalar, ty: Ty<'tcx>, span: Span) -> Operand<'tcx> {
2020-01-11T23:42:32.5154454Z     |                            ^^^^^ not valid as function parameter
2020-01-11T23:42:32.5155020Z     = note: `self` is only valid as the first parameter of an associated function
2020-01-11T23:42:32.5155061Z 
2020-01-11T23:42:32.5243163Z error: unexpected `self` parameter in function
2020-01-11T23:42:32.5243480Z    --> src/librustc_mir/transform/const_prop.rs:650:9
---
2020-01-11T23:42:32.5246407Z     |                          ^^^^^^^^^ not valid as function parameter
2020-01-11T23:42:32.5246629Z     |
2020-01-11T23:42:32.5247090Z     = note: `self` is only valid as the first parameter of an associated function
2020-01-11T23:42:32.5247165Z 
2020-01-11T23:42:32.5247539Z error: expected one of `async`, `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, or `unsafe`, found keyword `enum`
2020-01-11T23:42:32.5248172Z     |
2020-01-11T23:42:32.5248458Z 749 | #[derive(Clone, Copy, Debug, PartialEq)]
2020-01-11T23:42:32.5248813Z     |                                         - expected one of 9 possible tokens
2020-01-11T23:42:32.5248813Z     |                                         - expected one of 9 possible tokens
2020-01-11T23:42:32.5249104Z 750 | enum ConstPropMode {
2020-01-11T23:42:32.5249437Z 
2020-01-11T23:42:33.5287033Z error[E0433]: failed to resolve: use of undeclared type or module `CanConstProp`
2020-01-11T23:42:33.5287540Z    --> src/librustc_mir/transform/const_prop.rs:312:30
2020-01-11T23:42:33.5287814Z     |
2020-01-11T23:42:33.5287814Z     |
2020-01-11T23:42:33.5288119Z 312 |         let can_const_prop = CanConstProp::check(body);
2020-01-11T23:42:33.5288508Z     |                              ^^^^^^^^^^^^ use of undeclared type or module `CanConstProp`
2020-01-11T23:42:33.5321863Z error[E0433]: failed to resolve: use of undeclared type or module `PlaceBase`
2020-01-11T23:42:33.5322392Z    --> src/librustc_mir/transform/const_prop.rs:612:41
2020-01-11T23:42:33.5322611Z     |
2020-01-11T23:42:33.5322611Z     |
2020-01-11T23:42:33.5322941Z 612 |                 if let PlaceRef { base: PlaceBase::Local(local), .. } = place_ref.as_ref() {
2020-01-11T23:42:33.5323379Z 
2020-01-11T23:42:33.6037448Z error[E0412]: cannot find type `ConstPropMode` in this scope
2020-01-11T23:42:33.6037880Z    --> src/librustc_mir/transform/const_prop.rs:257:37
2020-01-11T23:42:33.6038114Z     |
2020-01-11T23:42:33.6038114Z     |
2020-01-11T23:42:33.6038383Z 41  | pub struct ConstProp;
2020-01-11T23:42:33.6038781Z     | --------------------- similarly named struct `ConstProp` defined here
2020-01-11T23:42:33.6038996Z ...
2020-01-11T23:42:33.6039291Z 257 |     can_const_prop: IndexVec<Local, ConstPropMode>,
2020-01-11T23:42:33.6039767Z 
2020-01-11T23:42:33.8804804Z error: unused import: `PanicInfo`
2020-01-11T23:42:33.8805183Z  --> src/librustc_mir/transform/const_prop.rs:7:43
2020-01-11T23:42:33.8805402Z   |
2020-01-11T23:42:33.8805402Z   |
2020-01-11T23:42:33.8805978Z 7 | use rustc::mir::interpret::{InterpResult, PanicInfo, Scalar};
2020-01-11T23:42:33.8806334Z   |                                           ^^^^^^^^^
2020-01-11T23:42:33.8806555Z   |
2020-01-11T23:42:33.8806918Z   = note: `-D unused-imports` implied by `-D warnings`
2020-01-11T23:42:33.8807079Z 
2020-01-11T23:42:33.8828954Z error: unused imports: `MutatingUseContext`, `NonMutatingUseContext`, `PlaceContext`
2020-01-11T23:42:33.8829524Z   |
2020-01-11T23:42:33.8829524Z   |
2020-01-11T23:42:33.8829850Z 9 |     MutVisitor, MutatingUseContext, NonMutatingUseContext, PlaceContext, Visitor,
2020-01-11T23:42:33.8830320Z 
2020-01-11T23:42:33.8920285Z error: unused imports: `LocalKind`, `Location`, `StatementKind`, `Statement`, `TerminatorKind`, `Terminator`
2020-01-11T23:42:33.8920664Z   --> src/librustc_mir/transform/const_prop.rs:13:23
2020-01-11T23:42:33.8921093Z    |
2020-01-11T23:42:33.8921093Z    |
2020-01-11T23:42:33.8921429Z 13 |     Local, LocalDecl, LocalKind, Location, Operand, Place, PlaceRef, ReadOnlyBodyAndCache, Rvalue,
2020-01-11T23:42:33.8922096Z 14 |     SourceInfo, SourceScope, SourceScopeData, Statement, StatementKind, Terminator, TerminatorKind,
2020-01-11T23:42:33.8922523Z    |                                               ^^^^^^^^^  ^^^^^^^^^^^^^  ^^^^^^^^^^  ^^^^^^^^^^^^^^
2020-01-11T23:42:33.8922607Z 
2020-01-11T23:42:45.5912108Z error[E0599]: no method named `visit_body` found for type `transform::const_prop::ConstPropagator<'_, '_>` in the current scope
2020-01-11T23:42:45.5912108Z error[E0599]: no method named `visit_body` found for type `transform::const_prop::ConstPropagator<'_, '_>` in the current scope
2020-01-11T23:42:45.5913327Z    --> src/librustc_mir/transform/const_prop.rs:97:29
2020-01-11T23:42:45.5913897Z     |
2020-01-11T23:42:45.5914492Z 97  |         optimization_finder.visit_body(body);
2020-01-11T23:42:45.5915337Z     |                             ^^^^^^^^^^ method not found in `transform::const_prop::ConstPropagator<'_, '_>`
2020-01-11T23:42:45.5915927Z ...
2020-01-11T23:42:45.5916491Z 253 | struct ConstPropagator<'mir, 'tcx> {
2020-01-11T23:42:45.5917147Z     | ---------------------------------- method `visit_body` not found for this
2020-01-11T23:42:45.5918286Z     = help: items from traits can only be used if the trait is implemented and in scope
2020-01-11T23:42:45.5918286Z     = help: items from traits can only be used if the trait is implemented and in scope
2020-01-11T23:42:45.5918900Z     = note: the following traits define an item `visit_body`, perhaps you need to implement one of them:
2020-01-11T23:42:45.5919418Z             candidate #1: `rustc::mir::visit::Visitor`
2020-01-11T23:42:45.5920138Z             candidate #2: `rustc::mir::visit::MutVisitor`
2020-01-11T23:42:45.5920732Z             candidate #3: `rustc_hir::intravisit::Visitor`
2020-01-11T23:42:45.6317932Z error[E0026]: struct `rustc::mir::PlaceRef` does not have a field named `base`
2020-01-11T23:42:45.6319551Z    --> src/librustc_mir/transform/const_prop.rs:612:35
2020-01-11T23:42:45.6320206Z     |
2020-01-11T23:42:45.6320206Z     |
2020-01-11T23:42:45.6321450Z 612 |                 if let PlaceRef { base: PlaceBase::Local(local), .. } = place_ref.as_ref() {
2020-01-11T23:42:45.6322124Z     |                                   ^^^^ struct `rustc::mir::PlaceRef` does not have this field
2020-01-11T23:42:45.6346745Z error[E0308]: mismatched types
2020-01-11T23:42:45.6347342Z    --> src/librustc_mir/transform/const_prop.rs:562:10
2020-01-11T23:42:45.6347755Z     |
2020-01-11T23:42:45.6348215Z 556 |     fn const_prop(
2020-01-11T23:42:45.6348215Z 556 |     fn const_prop(
2020-01-11T23:42:45.6348768Z     |        ---------- implicitly returns `()` as its body has no tail or `return` expression
2020-01-11T23:42:45.6349582Z 562 |     ) -> Option<()> {
2020-01-11T23:42:45.6350088Z     |          ^^^^^^^^^^ expected enum `std::option::Option`, found `()`
2020-01-11T23:42:45.6350494Z     |
2020-01-11T23:42:45.6350966Z     = note:   expected enum `std::option::Option<()>`
2020-01-11T23:42:45.6350966Z     = note:   expected enum `std::option::Option<()>`
2020-01-11T23:42:45.6351398Z             found unit type `()`
2020-01-11T23:42:45.6351564Z 
2020-01-11T23:42:46.6365514Z error: unused import: `MutVisitor`
2020-01-11T23:42:46.6366509Z  --> src/librustc_mir/transform/const_prop.rs:9:5
2020-01-11T23:42:46.6367085Z   |
2020-01-11T23:42:46.6367693Z 9 |     MutVisitor, MutatingUseContext, NonMutatingUseContext, PlaceContext, Visitor,
2020-01-11T23:42:46.6368556Z 
2020-01-11T23:42:46.6369212Z error: unused import: `Visitor`
2020-01-11T23:42:46.6369745Z  --> src/librustc_mir/transform/const_prop.rs:9:74
2020-01-11T23:42:46.6370229Z   |
2020-01-11T23:42:46.6370229Z   |
2020-01-11T23:42:46.6370836Z 9 |     MutVisitor, MutatingUseContext, NonMutatingUseContext, PlaceContext, Visitor,
2020-01-11T23:42:46.6371674Z 
2020-01-11T23:42:46.6944266Z error: aborting due to 18 previous errors
2020-01-11T23:42:46.6944369Z 
2020-01-11T23:42:46.6944654Z Some errors have detailed explanations: E0026, E0308, E0412, E0433, E0599.
2020-01-11T23:42:46.6944654Z Some errors have detailed explanations: E0026, E0308, E0412, E0433, E0599.
2020-01-11T23:42:46.6944962Z For more information about an error, try `rustc --explain E0026`.
2020-01-11T23:42:46.7127978Z error: could not compile `rustc_mir`.
2020-01-11T23:42:46.7149085Z warning: build failed, waiting for other jobs to finish...
2020-01-11T23:42:52.0118881Z error: build failed
2020-01-11T23:42:52.0151051Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-11T23:42:52.0165117Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-01-11T23:42:52.0165559Z Build completed unsuccessfully in 0:05:21
2020-01-11T23:42:52.0216708Z == clock drift check ==
2020-01-11T23:42:52.0227899Z   local time: Sat Jan 11 23:42:52 UTC 2020
2020-01-11T23:42:52.0227899Z   local time: Sat Jan 11 23:42:52 UTC 2020
2020-01-11T23:42:52.5441536Z   network time: Sat, 11 Jan 2020 23:42:52 GMT
2020-01-11T23:42:52.5441659Z == end clock drift check ==
2020-01-11T23:42:52.9273427Z 
2020-01-11T23:42:52.9361908Z ##[error]Bash exited with code '1'.
2020-01-11T23:42:52.9396981Z ##[section]Starting: Checkout
2020-01-11T23:42:52.9398804Z ==============================================================================
2020-01-11T23:42:52.9398859Z Task         : Get sources
2020-01-11T23:42:52.9398920Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
