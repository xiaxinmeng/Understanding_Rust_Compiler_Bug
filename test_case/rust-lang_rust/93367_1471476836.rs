rust
auto trait NotEq {}
struct TypeCmp<U, V>(U, V);
impl<T> !NotEq for TypeComp<T, T> {}

impl<T> SomeTrait<T> for MyStruct
where
    T: Default,
    TypeCmp<T, u8>: NotEq,
    TypeCmp<T, u16>: NotEq,
    // More types that I want to exclude
{
    // ... trait impl
}
