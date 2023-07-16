plain
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0308]: arguments to this function are incorrect
   --> compiler/rustc_middle/src/middle/privacy.rs:126:21
    |
126 |             if tree.is_descendant_of(current_restricted_id, restricted_id) {
    |                     ^^^^^^^^^^^^^^^^ ---------------------  ------------- expected struct `rustc_span::def_id::DefId`, found struct `rustc_span::def_id::LocalDefId`
    |                                      expected struct `rustc_span::def_id::DefId`, found struct `rustc_span::def_id::LocalDefId`
    |
note: associated function defined here
   --> compiler/rustc_middle/src/ty/mod.rs:339:8
   --> compiler/rustc_middle/src/ty/mod.rs:339:8
    |
339 |     fn is_descendant_of(self, mut descendant: DefId, ancestor: DefId) -> bool {

error[E0308]: arguments to this function are incorrect
   --> compiler/rustc_middle/src/middle/privacy.rs:160:29
    |
    |
160 |                     if tree.is_descendant_of(current_restricted_id, inherited_restricted_id) {
    |                             ^^^^^^^^^^^^^^^^ ---------------------  ----------------------- expected struct `rustc_span::def_id::DefId`, found struct `rustc_span::def_id::LocalDefId`
    |                                              expected struct `rustc_span::def_id::DefId`, found struct `rustc_span::def_id::LocalDefId`
    |
note: associated function defined here
   --> compiler/rustc_middle/src/ty/mod.rs:339:8
   --> compiler/rustc_middle/src/ty/mod.rs:339:8
    |
339 |     fn is_descendant_of(self, mut descendant: DefId, ancestor: DefId) -> bool {

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_middle` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
