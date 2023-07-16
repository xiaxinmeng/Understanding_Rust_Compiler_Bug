plain
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
error[E0308]: mismatched types
   --> compiler/rustc_infer/src/infer/canonical/query_response.rs:506:71
    |
506 |                 .extend(self.at(cause, param_env).define_opaque_types(true).eq(a, b)?.obligations);
    |                                                   ------------------- ^^^^ expected enum `DefiningAnchor`, found `bool`
    |                                                   arguments to this method are incorrect
    |
note: associated function defined here
   --> compiler/rustc_infer/src/infer/at.rs:92:12
   --> compiler/rustc_infer/src/infer/at.rs:92:12
    |
92  |     pub fn define_opaque_types(self, define_opaque_types: DefiningAnchor) -> Self {

    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
For more information about this error, try `rustc --explain E0308`.
