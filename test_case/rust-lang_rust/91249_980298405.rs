rust
#![feature(decl_macro)]

pub macro generate_class_new($name: ident, $new: ident) {
    pub struct $name {
        x: i32
    }

    impl $name {
        pub fn $new(x: i32) -> Self {
            Self { x }
        }
    }
}

generate_class_new!(MyTestClass, new);

fn main() {
    let instance = MyTestClass::new(3);
}
