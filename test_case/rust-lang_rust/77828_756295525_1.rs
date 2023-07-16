
> [src/librustdoc/visit_ast.rs:86] self.cx.tcx.parent_module(def.hir_id) = DefId(0:0 ~ core[8787])
> [src/librustdoc/visit_ast.rs:87] {
>     use rustc_middle::ty::DefIdTree;
>     self.cx.tcx.parent(self.cx.tcx.hir().local_def_id(def.hir_id).to_def_id()).unwrap()
> } = DefId(0:16466 ~ core[8787]::C)
> 