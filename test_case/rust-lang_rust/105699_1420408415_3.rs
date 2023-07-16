rust
fn works(arg: &Box<dyn Drop>) -> Option<&dyn Drop> {
    true.then(|| { arg.as_ref() })
}
fn also_works(arg: &Box<dyn Drop>) -> Option<&dyn Drop> {
    true.then(|| arg.as_ref())
}
