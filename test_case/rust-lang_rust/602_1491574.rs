
fn unify_step(&@ctxt cx, &t expected, &t actual) -> result {
    // TODO: occurs check, to make sure we don't loop forever when
    // unifying e.g. 'a and option['a]
