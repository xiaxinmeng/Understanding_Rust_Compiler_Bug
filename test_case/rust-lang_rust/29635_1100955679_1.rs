rust
struct Bar;

// This is legal, because S is Bar and we own Bar
impl Clone for a::Foo<Bar, ()> {
    fn clone(&self) -> Self {
        a::Foo {
            // PhantomData ceremony
        }
    }
}

// This is illegal, because Bar is in T. S is () and we don't own ()
impl Clone for a::Foo<(), Bar> {
    fn clone(&self) -> Self {
        a::Foo {
            // PhantomData ceremony
        }
    }
}
