plain
   |     |                  help: a similar name exists in the module: `Layout`
   |     no `LayoutOf` in `abi`

    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error[E0599]: no method named `layout_of` found for reference `&'a <Bx as HasCodegen<'tcx>>::CodegenCx` in the current scope
   --> compiler/rustc_codegen_ssa/src/mir/debuginfo.rs:376:61
    |
376 | ...                   let var_ty_layout = self.cx.layout_of(var_ty);
    |                                                   ^^^^^^^^^ method not found in `&'a <Bx as HasCodegen<'tcx>>::CodegenCx`
    = help: items from traits can only be used if the trait is in scope
    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
            `use crate::rustc_middle::ty::layout::LayoutOf;`

