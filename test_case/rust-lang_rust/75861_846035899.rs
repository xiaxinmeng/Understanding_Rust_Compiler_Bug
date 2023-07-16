rust
fn new_cyclic2<A, B>(f: impl FnOnce(&Weak<A>, &Weak<B>) -> (A, B)) -> (Arc<A>, Arc<B>) {
    let mut b: Option<Arc<B>> = None;
    let a = Arc::new_cyclic(|weak_a| {
        let mut a : Option<A> = None;
        b = Some(Arc::new_cyclic(|weak_b| {
            let (a2, b2) = f(weak_a, weak_b);
            a = Some(a2);
            b2
        }));
        a.unwrap()
    });
    (a, b.unwrap())
}
