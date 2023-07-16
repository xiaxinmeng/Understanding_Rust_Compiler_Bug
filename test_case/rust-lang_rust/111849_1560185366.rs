rust
struct Cyclic {
    this: Weak<Cyclic>,
}

let mut cycle = UniqueRc::new(Cyclic { this: Weak::new() };
cycle.this = UniqueRc::weak(cycle);

// convert to a normal `Rc`
let rc = UniqueRc::into_rc(cycle);
