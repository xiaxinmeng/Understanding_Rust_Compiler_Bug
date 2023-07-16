rust
    trait Trait { type Assoc; }
    
    struct Foo<T : Trait>(T::Assoc);
    
    impl Trait for fn(&'static ()) {
        type Assoc = ();
    }

    let _: &dyn Send = &async {
        let _it = Foo::<fn(&'static ())>(());
        async {}.await;
    };
    