
fn vacuous<A>(o: Option<Void>) -> Option<A> {
    match self {
        None => None,
        _ => unreachable!()
    }
}
