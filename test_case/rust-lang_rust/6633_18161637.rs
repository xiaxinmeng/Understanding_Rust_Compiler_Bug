 rust
use foo::name::name; // should be unused
use foo::name;       // warning emits here instead

pub mod foo {
    pub mod name {
        pub type a = int;
        pub mod name {
            pub type a = float;
        }
    }
}

fn bar() -> name::a { 1 }

fn main(){}
