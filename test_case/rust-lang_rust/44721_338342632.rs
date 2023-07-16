
let context = opt_def_id.map(|def_id| TypeLoweringContext::FnParameter(def_id))
    .unwrap_or(TypeLoweringContext::Other);
