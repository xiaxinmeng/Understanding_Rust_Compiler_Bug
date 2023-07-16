rust
trait MyFn<'a> {}
impl<'a, F> MyFn<'a> for F where
    F: FnOnce(&'a i32) -> &'a i32 {}

fn foo<F>(f: F) where
    F: for<'a> FnOnce(&'a i32) -> &'a i32, // <-- extra bound
    F: for<'a> MyFn<'a> {}

fn main() {
    foo(|x: &i32| -> &i32 { x }); // OK
}
