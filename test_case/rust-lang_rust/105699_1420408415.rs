rust
fn fails(arg: &mut Box<dyn Drop>) -> Option<&mut dyn Drop> {
    true.then(|| arg.as_mut())
}
fn works(arg: &mut Box<dyn Drop>) -> Option<&mut dyn Drop> {
    true.then(|| { arg.as_mut() })
}
