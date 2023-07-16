
// `Sized` bound is not implicitly added
fn f<FreshParameter: OtherBounds1>() where ExistingType: OtherBounds2 { ... }
