rust
mod some_module {
    pub trait Foo {
        fn foo(&self);
    }
    
    pub trait Bar {
        fn bar(&self);
    }
    
    pub struct Qux;
    
    impl Foo for Qux {
        fn foo(&self) {}
    }
    
    impl Bar for Qux {
        fn bar(&self) {}
    }
    
    pub trait Baz = Foo + Bar;
}

use some_module::{Baz, Qux};

fn use_baz(object: Qux) {
    // Should this work because Baz is in scope?
    object.foo();
    object.bar();
}
