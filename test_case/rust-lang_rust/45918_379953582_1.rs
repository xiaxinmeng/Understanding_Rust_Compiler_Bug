rust
fn foo<T>(_x: T) where T: Bar<impl Clone> {
//^ `impl Trait` not allowed outside of function and inherent method return types
}
