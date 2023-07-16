
#[safe_suggestion]
fn f() {}

// in case we have multiple suggestions
#[safe_suggestion="this function does this"]
fn f() {}

#[safe_suggestion="this other function does that"]
fn f2() {}
