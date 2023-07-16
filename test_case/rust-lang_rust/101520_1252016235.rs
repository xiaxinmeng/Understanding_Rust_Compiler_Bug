rust
struct Foo<'a, T: ?Sized>(&'a T, i32);

fn foo<'a, 'b, T: ?Sized>(x: Foo<'a, T>) -> Foo<'b, T> {
    unsafe { std::mem::transmute(x) }
    // ^ only an error if the `i32` field is there, otherwise compiles fine
}
