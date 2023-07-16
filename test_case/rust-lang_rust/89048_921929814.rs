plain
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error: attempting to skip non-existent parameter
  --> compiler/rustc_borrowck/src/type_check/relate_tys.rs:20:55
   |
20 | #[instrument(skip(infcx, param_env, borrowck_context, universal_region_relations), level = "debug")]

error: could not compile `rustc_borrowck` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
