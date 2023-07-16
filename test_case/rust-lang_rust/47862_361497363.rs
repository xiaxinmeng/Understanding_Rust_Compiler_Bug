rust
if let Ok(new_n) = cx.tcx.const_eval(param_env.and((def_id, substs))) {
    n = new_n;
}
