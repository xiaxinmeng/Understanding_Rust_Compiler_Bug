rust
trait Trait {
    fn f(&mut self);
}

fn g<F: FnOnce(&mut dyn Trait)>(f: F, v: &mut dyn Trait) {
    f(v)
}

fn f(v: &mut dyn Trait) {
    g(Trait::f, v)
}
