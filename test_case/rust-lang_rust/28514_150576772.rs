 rust
fn use_c<T: C>(x: &T) {
    // A is private
    // B is pub, not reexported
    // C : A + B is pub, reexported

    x.a(); // can call
    x.b(); // can call
    x.c(); // ok

    C::a(x); // can call
    C::b(x); // can call
    C::c(x); // ok
}
