rust
fn do_a_thing<T>(foo: T) -> T {
    struct Closure<U>(U);

    impl<U> FnOnce<()> for Closure<U> {
        type Output = U;

        fn call_once(self, _: ()) -> U {
            self.0
        }
    }

    let my_fn = Closure::<T>(foo);

    my_fn()
}
