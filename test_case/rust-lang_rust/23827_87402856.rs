 rust
impl<C: Component> FnOnce<(C,)> for Prototype {
    type Output = Prototype;

    extern "rust-call" fn call_once(self, (comp,): (C,)) -> Prototype {
        Fn::call(&self, (comp,))
    }
}
