plain
     |
2786 |         projection_ty: ty::ProjectionTy<'tcx>,
     |         ^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_projection_ty`
     |
     = note: `-D unused-variables` implied by `-D warnings`
error: unused variable: `ty`
    --> compiler/rustc_infer/src/infer/error_reporting/mod.rs:2787:9
     |
2787 |         ty: Ty<'tcx>,
