plain
    Checking rustc_expand v0.0.0 (/checkout/compiler/rustc_expand)
error[E0424]: expected value, found module `self`
    --> compiler/rustc_middle/src/ty/context.rs:1363:15
     |
1362 |     pub fn async_kind(def_id: DefId) -> Option<hir::AsyncGeneratorKind> {
     |            ---------- this function doesn't have a `self` parameter
1363 |         match self.generator_kind(def_id) {
     |               ^^^^ `self` value is a keyword only available in methods with a `self` parameter
     |
help: add a `self` receiver parameter to make the associated `fn` a method
     |
1362 |     pub fn async_kind(&self, def_id: DefId) -> Option<hir::AsyncGeneratorKind> {

    Checking rustc_builtin_macros v0.0.0 (/checkout/compiler/rustc_builtin_macros)
For more information about this error, try `rustc --explain E0424`.
error: could not compile `rustc_middle` due to previous error
