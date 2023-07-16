rust
pub enum Foo<'a, T> {
    Struct {},
    Tuple(),
    Unit,
    Usage(&'a T),
}
pub fn main() {
    let foo = Foo::<String>::Unit;
    let foo = Foo::<String>::Tuple();
    let foo = Foo::<String>::Struct {};
    let foo = Foo::Unit::<String>;
    let foo = Foo::Tuple::<String>();
    let foo = Foo::Struct::<String> {};
}
