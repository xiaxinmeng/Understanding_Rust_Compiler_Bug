plain
    Checking rustc_middle v0.0.0 (/checkout/compiler/rustc_middle)
    Checking rustc_ast_passes v0.0.0 (/checkout/compiler/rustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
error[E0277]: `impl Fn(ItemId)` cannot be shared between threads safely
   --> compiler/rustc_middle/src/hir/mod.rs:67:9
    |
67  |         par_for_each_in(&self.items[..], |&id| f(id))
    |         ^^^^^^^^^^^^^^^                  ----------- within this `[closure@compiler/rustc_middle/src/hir/mod.rs:67:42: 67:53]`
    |         |
    |         `impl Fn(ItemId)` cannot be shared between threads safely
    |
    = note: required because it appears within the type `&impl Fn(ItemId)`
    = note: required because it appears within the type `[closure@compiler/rustc_middle/src/hir/mod.rs:67:42: 67:53]`
note: required by a bound in `par_for_each_in`
    |
    |
340 |             for_each: impl Fn(T::Item) + Sync + Send,
    |                                          ^^^^ required by this bound in `par_for_each_in`
    |
    |
66  |     pub fn par_items(&self, f: impl Fn(ItemId) + std::marker::Sync) {


error[E0277]: `impl Fn(TraitItemId)` cannot be shared between threads safely
   --> compiler/rustc_middle/src/hir/mod.rs:71:9
    |
71  |         par_for_each_in(&self.trait_items[..], |&id| f(id))
    |         ^^^^^^^^^^^^^^^                        ----------- within this `[closure@compiler/rustc_middle/src/hir/mod.rs:71:48: 71:59]`
    |         |
    |         `impl Fn(TraitItemId)` cannot be shared between threads safely
    |
    = note: required because it appears within the type `&impl Fn(TraitItemId)`
    = note: required because it appears within the type `[closure@compiler/rustc_middle/src/hir/mod.rs:71:48: 71:59]`
note: required by a bound in `par_for_each_in`
    |
    |
340 |             for_each: impl Fn(T::Item) + Sync + Send,
    |                                          ^^^^ required by this bound in `par_for_each_in`
    |
    |
70  |     pub fn par_trait_items(&self, f: impl Fn(TraitItemId) + std::marker::Sync) {


error[E0277]: `impl Fn(ImplItemId)` cannot be shared between threads safely
   --> compiler/rustc_middle/src/hir/mod.rs:75:9
    |
75  |         par_for_each_in(&self.impl_items[..], |&id| f(id))
    |         ^^^^^^^^^^^^^^^                       ----------- within this `[closure@compiler/rustc_middle/src/hir/mod.rs:75:47: 75:58]`
    |         |
    |         `impl Fn(ImplItemId)` cannot be shared between threads safely
    |
    = note: required because it appears within the type `&impl Fn(ImplItemId)`
    = note: required because it appears within the type `[closure@compiler/rustc_middle/src/hir/mod.rs:75:47: 75:58]`
note: required by a bound in `par_for_each_in`
    |
    |
340 |             for_each: impl Fn(T::Item) + Sync + Send,
    |                                          ^^^^ required by this bound in `par_for_each_in`
    |
    |
74  |     pub fn par_impl_items(&self, f: impl Fn(ImplItemId) + std::marker::Sync) {


error[E0277]: `impl Fn(ForeignItemId)` cannot be shared between threads safely
   --> compiler/rustc_middle/src/hir/mod.rs:79:9
    |
79  |         par_for_each_in(&self.foreign_items[..], |&id| f(id))
    |         ^^^^^^^^^^^^^^^                          ----------- within this `[closure@compiler/rustc_middle/src/hir/mod.rs:79:50: 79:61]`
    |         |
    |         `impl Fn(ForeignItemId)` cannot be shared between threads safely
    |
    = note: required because it appears within the type `&impl Fn(ForeignItemId)`
    = note: required because it appears within the type `[closure@compiler/rustc_middle/src/hir/mod.rs:79:50: 79:61]`
note: required by a bound in `par_for_each_in`
    |
    |
340 |             for_each: impl Fn(T::Item) + Sync + Send,
    |                                          ^^^^ required by this bound in `par_for_each_in`
    |
    |
78  |     pub fn par_foreign_items(&self, f: impl Fn(ForeignItemId) + std::marker::Sync) {

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_middle` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
