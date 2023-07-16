plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0599]: no variant or associated item named `Invisible` found for enum `ty::Visibility` in the current scope
   --> compiler/rustc_middle/src/middle/privacy.rs:40:48
    |
40  |         EffectiveVisibility { vis: Visibility::Invisible }
    |                                                ^^^^^^^^^ variant or associated item not found in `ty::Visibility`
   ::: compiler/rustc_middle/src/ty/mod.rs:266:1
    |
266 | pub enum Visibility {
266 | pub enum Visibility {
    | ------------------- variant or associated item `Invisible` not found for this enum

error[E0599]: no variant or associated item named `Invisible` found for enum `ty::Visibility` in the current scope
   --> compiler/rustc_middle/src/middle/privacy.rs:48:33
    |
48  |         self.vis != Visibility::Invisible
    |                                 ^^^^^^^^^ variant or associated item not found in `ty::Visibility`
   ::: compiler/rustc_middle/src/ty/mod.rs:266:1
    |
266 | pub enum Visibility {
266 | pub enum Visibility {
    | ------------------- variant or associated item `Invisible` not found for this enum

error[E0599]: no variant or associated item named `Invisible` found for enum `ty::Visibility` in the current scope
   --> compiler/rustc_middle/src/middle/privacy.rs:161:25
161 |             Visibility::Invisible => None,
    |                         ^^^^^^^^^ variant or associated item not found in `ty::Visibility`
    |
   ::: compiler/rustc_middle/src/ty/mod.rs:266:1
   ::: compiler/rustc_middle/src/ty/mod.rs:266:1
    |
266 | pub enum Visibility {
    | ------------------- variant or associated item `Invisible` not found for this enum
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_middle` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_middle` due to 3 previous errors
