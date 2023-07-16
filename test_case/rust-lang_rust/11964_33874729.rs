 rust
#[feature(default_type_params)];
struct Foo<T>(T);
struct Bar<T, U = Foo<T>>(T, U); 
fn main() {
    let _: Bar<bool> = Bar(true, Foo(false));
}
// error: mismatched types: expected `Bar<bool>` but found `Bar<bool,Foo<bool>>` (expected type parameter but found bool)
