rust
#![feature(const_generics)]

struct Foo<const T: usize>;
struct Bar;

const C: usize = {
    impl Bar { const fn a() -> usize { 42 } };
    Bar::a() + Bar::b()
};

impl Foo<{
    mod m { impl super::Bar { pub const fn b() -> usize { 42 } } }
    C
}> {}
