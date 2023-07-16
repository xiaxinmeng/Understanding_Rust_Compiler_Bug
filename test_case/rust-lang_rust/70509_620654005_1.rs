rust
impl PartialEq for Foo {
    fn eq(&self, rhs: &Self) -> bool {
        use core::intrinsics::discriminant_value;
        if discriminant_value(self) != discriminant_value(rhs) {
            return false;
        }

        match (self, rhs) {
            (Foo::A(l), Foo::A(r)) => l == r,
            (Foo::B, Foo::B) => true,
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}

