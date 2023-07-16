rust
mod not_in_scope {
    pub trait Foo {
        fn foo(&self);
    }
    
    impl Foo for () {
        fn foo(&self) {}
    }
}

fn existential_impl_trait() {
    fn f() -> impl not_in_scope::Foo {
        ()
    }
    
    f().foo();
}

fn universal_impl_trait(x: impl not_in_scope::Foo) {
    x.foo();
}

fn dyn_trait(x: &dyn not_in_scope::Foo) {
    x.foo();
}

fn trait_bound<T: not_in_scope::Foo>(x: T) {
    x.foo();
}

pub fn main() {
    existential_impl_trait();
    dyn_trait(&());
    universal_impl_trait(());
    trait_bound(());
}
