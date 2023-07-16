 rust
struct Bar;
use foo::Baz;
//^ here, we would try to resolve `foo::Baz`. Since `foo::Baz` is not yet successful,
//| we would follow the only single import that can define it (`use self::Bar as Baz`) and
//| try to resolve `foo::Bar`, which would incorrectly fail since `foo` is already borrowed
//| (causing the above import to incorrectly fail).
mod foo {
    use self::Bar as Baz;
    use Bar;
}
