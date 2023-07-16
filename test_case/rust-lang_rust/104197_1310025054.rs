
fn do_something<'a, P: View>(parent: &Hierarchical<P>) {
    let h: &'a dyn View = parent;
}
