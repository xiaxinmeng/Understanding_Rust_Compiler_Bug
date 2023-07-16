 rust
mod mod1 {
    struct Foo(u32);
}

mod mod2 {
    struct Foo(i64);
}

mod mod3 {
    use mod1::Foo;

    fn bar() -> usize { 
        mem::size_of::<Foo>()
    }
}
