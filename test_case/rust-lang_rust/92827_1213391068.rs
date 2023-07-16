rs
trait Foo{
    const DECIDER: bool;
}
trait Bar{ /*stuff*/ }
impl<T: Foo<DECIDER = true>> Bar for T{ /*stuff*/ }
impl<T: Foo<DECIDER = false>> Bar for T{ /*stuff*/ }
