
let callee_features = &tcx.codegen_fn_attrs(def_id).target_features;
let self_features = &tcx.body_codegen_attrs(def_id.to_def_id()).target_features;
