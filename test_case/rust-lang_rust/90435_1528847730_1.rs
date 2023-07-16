rust
mod v1_0 {
    pub struct Foo(());
    pub struct Bar(i32);
    
    #[repr(transparent)]
    pub struct Murky {
        pub foo: Foo,
        pub bar: Bar,
    }
}

mod v1_1 {
    pub struct Foo(i32);
    pub struct Bar(());
    
    #[repr(transparent)]
    pub struct Murky {
        pub foo: Foo,
        pub bar: Bar,
    }
}
