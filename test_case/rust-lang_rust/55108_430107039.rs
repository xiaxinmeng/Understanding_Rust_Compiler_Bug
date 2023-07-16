
main.rs:
    mod foo;
    fn main() {
        foo::inline::inline2::bar::f();
    }

foo.rs:
    pub mod inline {
        pub mod inline2 {
            #[path="baz.rs"]
            pub mod bar;
        }
    }

inline/inline2/baz.rs:
    pub fn f () {}
