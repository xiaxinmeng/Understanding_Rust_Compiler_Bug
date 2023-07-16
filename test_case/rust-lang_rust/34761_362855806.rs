Rust
struct Cyclic<A, T> {
    cycle: RefCell<Option<Rc<Cyclic<T, A>, A>>,
    data: T
}

fn foo() {
    let arena = make_arena();
    let rc1 = arena <- Cyclic {
        cycle: Default::default(),
        data: Pin::new(),
    };
    
    // make a cycle    
    *rc1.cycle.borrow_mut() = Some(rc1.clone());
    rc1.data.pin();

    // at end-of-scope, no destructors are run, but the arena (and its containing Rc) are free-ed.
}
