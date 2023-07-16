rust
struct Rec /* = */ (
    pub
    Rc<Cell<Option<Rec>>>,
);

fn evil ()
{
    let this = Rec::new(); // Creates a cycle (thanks to shared mutability)
    this.0.read_with(|inner: &Option<Rec>| {
        // given the type of inner, it is illegal to mutate
        // the pointee (the Option), for instance, its
        // discriminant is known to be immutable.
        if let &Some(ref this) = inner {
            this.0.set(None);
            // Woops, we mutated an immutable discriminant: UB
        }
    })
}
