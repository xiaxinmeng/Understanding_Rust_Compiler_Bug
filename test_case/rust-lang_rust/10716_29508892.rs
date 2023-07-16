 rust
macro_rules! conflict {
     () => {
        use bar::foo; 
        fn inner() { foo::func() }
    }
}

mod foo { pub fn func() {} } // A

mod bar {
    pub mod foo { pub fn func() {} } // B
}

fn before() {
    foo::func();
}

conflict!()

fn after() {
    foo::func();
}
