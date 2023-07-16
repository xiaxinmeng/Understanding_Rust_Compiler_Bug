plain
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0107]: this struct takes 0 lifetime arguments but 1 lifetime argument was supplied
     |
2104 | ) -> ty::CaptureInfo<'tcx> {
     |          ^^^^^^^^^^^------ help: remove these generics
     |          |
---
     |
276  | pub struct CaptureInfo {
     |            ^^^^^^^^^^^

error[E0107]: this struct takes 0 lifetime arguments but 1 lifetime argument was supplied
     |
     |
2102 |     capture_info_a: ty::CaptureInfo<'tcx>,
     |                         ^^^^^^^^^^^------ help: remove these generics
     |                         expected 0 lifetime arguments
     |
note: struct defined here, with 0 lifetime parameters
    --> /checkout/compiler/rustc_middle/src/ty/closure.rs:276:12
    --> /checkout/compiler/rustc_middle/src/ty/closure.rs:276:12
     |
276  | pub struct CaptureInfo {
     |            ^^^^^^^^^^^

error[E0107]: this struct takes 0 lifetime arguments but 1 lifetime argument was supplied
     |
     |
2103 |     capture_info_b: ty::CaptureInfo<'tcx>,
     |                         ^^^^^^^^^^^------ help: remove these generics
     |                         expected 0 lifetime arguments
     |
note: struct defined here, with 0 lifetime parameters
    --> /checkout/compiler/rustc_middle/src/ty/closure.rs:276:12
