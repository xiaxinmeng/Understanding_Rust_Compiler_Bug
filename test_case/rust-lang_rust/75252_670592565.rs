rust
struct A<T>(T);
struct B<T>(T);

trait C {
    type Item;
}

impl<'a, T> C for &'a A<T> {
    type Item = &'a T;
}

impl<'a, T: 'a> C for &'a B<T> {
    type Item = &'a T;
}

trait Foo where for<'a> &'a Self: C<Item = &'a Self::Item> {
    type Item;
}

// Doesn't compile: suggest `T: 'a` is needed.
// fn foo<'a, T, I: 'a + C<Item = &'a T>>(i: I) {}

// Compiles
impl<T> Foo for A<T> {
    type Item = T;
}

// Doesn't compile: suggest `T: 'a` is needed???
// impl<T> Foo for B<T> {
//     type Item = T;
// }
