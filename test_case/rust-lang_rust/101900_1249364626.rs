plain
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0004]: non-exhaustive patterns: `Effect` not covered
    --> compiler/rustc_typeck/src/check/fn_ctxt/_impl.rs:1303:23
1303 |                 match param.kind {
1303 |                 match param.kind {
     |                       ^^^^^^^^^^ pattern `Effect` not covered
note: `GenericParamDefKind` defined here
    --> /checkout/compiler/rustc_middle/src/ty/generics.rs:17:5
     |
13   | pub enum GenericParamDefKind {
13   | pub enum GenericParamDefKind {
     | ----------------------------
...
17   |     Effect,
     |     ^^^^^^ not covered
     = note: the matched value is of type `GenericParamDefKind`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
1332 ~                     }
1333 +                     Effect => todo!()
     |


error[E0004]: non-exhaustive patterns: `Effect` not covered
     |
1795 |                     match param.kind {
1795 |                     match param.kind {
     |                           ^^^^^^^^^^ pattern `Effect` not covered
note: `GenericParamDefKind` defined here
    --> /checkout/compiler/rustc_middle/src/ty/generics.rs:17:5
     |
13   | pub enum GenericParamDefKind {
13   | pub enum GenericParamDefKind {
     | ----------------------------
...
17   |     Effect,
     |     ^^^^^^ not covered
     = note: the matched value is of type `GenericParamDefKind`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
1802 ~                         }
1803 +                         Effect => todo!()
     |


error[E0004]: non-exhaustive patterns: `Effect` not covered
     |
     |
1821 |         InternalSubsts::for_item(self.tcx, def_id, |param, _| match param.kind {
     |                                                                     ^^^^^^^^^^ pattern `Effect` not covered
note: `GenericParamDefKind` defined here
    --> /checkout/compiler/rustc_middle/src/ty/generics.rs:17:5
     |
13   | pub enum GenericParamDefKind {
13   | pub enum GenericParamDefKind {
     | ----------------------------
...
17   |     Effect,
     |     ^^^^^^ not covered
     = note: the matched value is of type `GenericParamDefKind`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
1836 ~             }
1837 +             Effect => todo!()
     |


error[E0004]: non-exhaustive patterns: `Effect` not covered
    |
280 |             match param.kind {
280 |             match param.kind {
    |                   ^^^^^^^^^^ pattern `Effect` not covered
note: `GenericParamDefKind` defined here
   --> /checkout/compiler/rustc_middle/src/ty/generics.rs:17:5
    |
13  | pub enum GenericParamDefKind {
13  | pub enum GenericParamDefKind {
    | ----------------------------
...
17  |     Effect,
    |     ^^^^^^ not covered
    = note: the matched value is of type `GenericParamDefKind`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
288 ~                 }
289 +                 Effect => todo!()
    |


error[E0004]: non-exhaustive patterns: `Effect` not covered
    |
320 |             match param.kind {
320 |             match param.kind {
    |                   ^^^^^^^^^^ pattern `Effect` not covered
note: `GenericParamDefKind` defined here
   --> /checkout/compiler/rustc_middle/src/ty/generics.rs:17:5
    |
13  | pub enum GenericParamDefKind {
13  | pub enum GenericParamDefKind {
    | ----------------------------
...
17  |     Effect,
    |     ^^^^^^ not covered
    = note: the matched value is of type `GenericParamDefKind`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
328 ~                 }
329 +                 Effect => todo!()
    |


error[E0004]: non-exhaustive patterns: `Effect` not covered
    --> compiler/rustc_typeck/src/check/wfcheck.rs:1313:15
1313 |         match param.kind {
1313 |         match param.kind {
     |               ^^^^^^^^^^ pattern `Effect` not covered
note: `GenericParamDefKind` defined here
    --> /checkout/compiler/rustc_middle/src/ty/generics.rs:17:5
     |
13   | pub enum GenericParamDefKind {
13   | pub enum GenericParamDefKind {
     | ----------------------------
...
17   |     Effect,
     |     ^^^^^^ not covered
     = note: the matched value is of type `GenericParamDefKind`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
1346 ~             GenericParamDefKind::Lifetime => {}
1347 +             // Doesn't have defaults.
1348 +             Effect => todo!()
     |
     |

error[E0004]: non-exhaustive patterns: `Effect` not covered
    --> compiler/rustc_typeck/src/check/wfcheck.rs:1298:60
     |
1298 |     let is_our_default = |def: &ty::GenericParamDef| match def.kind {
     |                                                            ^^^^^^^^ pattern `Effect` not covered
note: `GenericParamDefKind` defined here
    --> /checkout/compiler/rustc_middle/src/ty/generics.rs:17:5
     |
13   | pub enum GenericParamDefKind {
13   | pub enum GenericParamDefKind {
     | ----------------------------
...
17   |     Effect,
     |     ^^^^^^ not covered
     = note: the matched value is of type `GenericParamDefKind`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
1303 ~         GenericParamDefKind::Lifetime => unreachable!(),
1304 ~         Effect => todo!(),
     |


error[E0004]: non-exhaustive patterns: `Effect` not covered
    --> compiler/rustc_typeck/src/check/wfcheck.rs:1359:15
1359 |         match param.kind {
1359 |         match param.kind {
     |               ^^^^^^^^^^ pattern `Effect` not covered
note: `GenericParamDefKind` defined here
    --> /checkout/compiler/rustc_middle/src/ty/generics.rs:17:5
     |
13   | pub enum GenericParamDefKind {
13   | pub enum GenericParamDefKind {
     | ----------------------------
...
17   |     Effect,
     |     ^^^^^^ not covered
     = note: the matched value is of type `GenericParamDefKind`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
1390 ~             }
1391 +             Effect => todo!()
     |


error[E0004]: non-exhaustive patterns: `Effect` not covered
   --> compiler/rustc_typeck/src/impl_wf_check.rs:121:15
121 |         match param.kind {
121 |         match param.kind {
    |               ^^^^^^^^^^ pattern `Effect` not covered
note: `GenericParamDefKind` defined here
   --> /checkout/compiler/rustc_middle/src/ty/generics.rs:17:5
    |
13  | pub enum GenericParamDefKind {
13  | pub enum GenericParamDefKind {
    | ----------------------------
...
17  |     Effect,
    |     ^^^^^^ not covered
    = note: the matched value is of type `GenericParamDefKind`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
152 ~             }
153 +             Effect => todo!()
    |

