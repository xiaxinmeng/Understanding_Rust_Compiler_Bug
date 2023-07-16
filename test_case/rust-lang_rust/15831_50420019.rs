 rust
extern foo<T>(t: T) {} // ok

#[no_mangle]
extern foo2<T>(t: T) {} // error

extern {
    fn bar<T>(); // error
}

fn main() {
    // ok
    let _f: extern fn(int) = foo;
    let _f: extern fn(i16) = foo;

    // ok
    fn inner<T>(f: extern fn(T), t: T) { f(t) }
}
