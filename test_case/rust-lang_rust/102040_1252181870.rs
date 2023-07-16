plain
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/obtain-borrowck/obtain-borrowck:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/obtain-borrowck/obtain-borrowck -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/obtain-borrowck/obtain-borrowck  driver.rs -o ""/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/obtain-borrowck/obtain-borrowck"/driver"
--- stderr -------------------------------
warning: ignoring --out-dir flag due to -o flag


error[E0277]: the trait bound `OwnerId: rustc_middle::ty::query::sealed::IntoQueryParam<DefId>` is not satisfied
   --> driver.rs:72:42
    |
72  |                 if matches!(tcx.def_kind(id.def_id), DefKind::Fn) {
    |                                 -------- ^^^^^^^^^ the trait `rustc_middle::ty::query::sealed::IntoQueryParam<DefId>` is not implemented for `OwnerId`
    |                                 required by a bound introduced by this call
    |
    |
    = help: the following other types implement trait `rustc_middle::ty::query::sealed::IntoQueryParam<P>`:
              <&'a P as rustc_middle::ty::query::sealed::IntoQueryParam<P>>
              <LocalDefId as rustc_middle::ty::query::sealed::IntoQueryParam<DefId>>
note: required by a bound in `rustc_middle::ty::query::<impl TyCtxt<'tcx>>::def_kind`
   --> /checkout/compiler/rustc_middle/src/ty/query.rs:375:40
    |
375 |     pub fn def_kind(self, def_id: impl IntoQueryParam<DefId>) -> DefKind {
    |                                        ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `rustc_middle::ty::query::<impl TyCtxt<'tcx>>::def_kind`

error[E0277]: the trait bound `OwnerId: rustc_middle::ty::query::sealed::IntoQueryParam<DefId>` is not satisfied
   --> driver.rs:78:42
    |
78  |                 if matches!(tcx.def_kind(id.def_id), DefKind::AssocFn) {
    |                                 -------- ^^^^^^^^^ the trait `rustc_middle::ty::query::sealed::IntoQueryParam<DefId>` is not implemented for `OwnerId`
    |                                 required by a bound introduced by this call
    |
    |
    = help: the following other types implement trait `rustc_middle::ty::query::sealed::IntoQueryParam<P>`:
              <&'a P as rustc_middle::ty::query::sealed::IntoQueryParam<P>>
              <LocalDefId as rustc_middle::ty::query::sealed::IntoQueryParam<DefId>>
note: required by a bound in `rustc_middle::ty::query::<impl TyCtxt<'tcx>>::def_kind`
   --> /checkout/compiler/rustc_middle/src/ty/query.rs:375:40
    |
375 |     pub fn def_kind(self, def_id: impl IntoQueryParam<DefId>) -> DefKind {
    |                                        ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `rustc_middle::ty::query::<impl TyCtxt<'tcx>>::def_kind`

error[E0277]: the trait bound `OwnerId: rustc_middle::ty::query::sealed::IntoQueryParam<DefId>` is not satisfied
   --> driver.rs:89:42
    |
89  |                 if matches!(tcx.def_kind(id.def_id), DefKind::AssocFn) {
    |                                 -------- ^^^^^^^^^ the trait `rustc_middle::ty::query::sealed::IntoQueryParam<DefId>` is not implemented for `OwnerId`
    |                                 required by a bound introduced by this call
    |
    |
    = help: the following other types implement trait `rustc_middle::ty::query::sealed::IntoQueryParam<P>`:
              <&'a P as rustc_middle::ty::query::sealed::IntoQueryParam<P>>
              <LocalDefId as rustc_middle::ty::query::sealed::IntoQueryParam<DefId>>
note: required by a bound in `rustc_middle::ty::query::<impl TyCtxt<'tcx>>::def_kind`
   --> /checkout/compiler/rustc_middle/src/ty/query.rs:375:40
    |
375 |     pub fn def_kind(self, def_id: impl IntoQueryParam<DefId>) -> DefKind {
    |                                        ^^^^^^^^^^^^^^^^^^^^^ required by this bound in `rustc_middle::ty::query::<impl TyCtxt<'tcx>>::def_kind`

error[E0277]: the trait bound `OwnerId: rustc_middle::ty::query::sealed::IntoQueryParam<DefId>` is not satisfied
   --> driver.rs:96:43
    |
96  |                 let _ = tcx.optimized_mir(def_id);
    |                             ------------- ^^^^^^ the trait `rustc_middle::ty::query::sealed::IntoQueryParam<DefId>` is not implemented for `OwnerId`
    |                             required by a bound introduced by this call
    |
    |
    = help: the following other types implement trait `rustc_middle::ty::query::sealed::IntoQueryParam<P>`:
              <&'a P as rustc_middle::ty::query::sealed::IntoQueryParam<P>>
              <LocalDefId as rustc_middle::ty::query::sealed::IntoQueryParam<DefId>>
note: required by a bound in `rustc_middle::ty::query::<impl TyCtxt<'tcx>>::optimized_mir`
   --> /checkout/compiler/rustc_middle/src/ty/query.rs:337:1
    |
337 | rustc_query_append! { define_callbacks! }
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `rustc_middle::ty::query::<impl TyCtxt<'tcx>>::optimized_mir`
    = note: this error originates in the macro `query_helper_param_ty` which comes from the expansion of the macro `rustc_query_append` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 4 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0277`.
make: *** [Makefile:19: all] Error 1
