 Rust
struct MutClosure<B, E> {
    body: B,
    environment: E
}
impl<B, E, P> FnOnce<P> for MutClosure<B,E> where B: Fn<(&mut E, P)> {
    type Output = B::Output;
    extern "rust-call" fn call_once(self, args: P) -> B::Output {
        self.body.call((&mut self.environment, args))
    }
}
impl<B, E, P> FnMut<P> for MutClosure<B,E> where B: Fn<(&mut E, P)> {
    extern "rust-call" fn call_mut(&mut self, args: P) -> B::Output {
        self.body.call((&mut self.environment, args))
    }
}
fn foo<T: Default>() {
    let foo = ...;
    let bar = ...;
    hof(|arg1, arg2| { /* closure body */ })
}
fn foo_desugared<T: Default>() {
    let foo = ...;
    let bar = ...;
    // it really doesn't matter if you pass extra early lifetimes in - they are substituted
    // early anyway.
    fn closure<'π, T: Default>((foo, bar): (&'π mut _, &'π mut _), (arg1, arg2): (_, _)) -> _ {
        // closure body
    }
    hof(MutClosure { body: closure::<T>, environment: (&mut foo, &mut bar) })
    //^ note we are creating a MutClosure<fn(...) {closure}, (_, _)>
}
