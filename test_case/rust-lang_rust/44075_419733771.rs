rust
let user_predicates = fcx.param_env.caller_bounds;
let wf_predicates = ty::wf::obligations(
    &fcx.infcx,
    fcx.param_env,
    fcx.body_id,
    item_ty,
    item.span,
).into_iter().flatten().map(|o| o.predicate).collect();
let wf_predicates = traits::elaborate_predicates(fcx.tcx, wf_predicates);
let wf_param_env = ty::ParamEnv {
    caller_bounds: fcx.tcx.mk_predicates(wf_predicates),
    reveal: fcx.param_env.reveal,
};
fcx.register_predicates(user_predicates.iter().map(|&predicate| {
    let code = ObligationCauseCode::MiscObligation;
    let cause = traits::ObligationCause::new(item.span, fcx.body_id, code);
    traits::Obligation::new(cause, wf_param_env, predicate)
}));
