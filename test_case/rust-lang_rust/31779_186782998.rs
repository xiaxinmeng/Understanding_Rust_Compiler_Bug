 rust
mod foo {
    mod bar {
        pub trait Parent {
            fn f(&self) {}
        }
    }
    pub trait Child: bar::Parent {}
}

fn f<T: foo::Child>(t: T) {
    t.f(); // Right now, this line causes an error that `Parent` is inaccessible since `bar` is private
}
