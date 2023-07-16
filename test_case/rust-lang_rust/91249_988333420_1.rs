rust
match! (new) { ($new:ident) => (
    pub macro generate_class_new($name: ident) {
        pub struct $name {
            x: i32
        }

        impl $name {
            pub fn $new(x: i32) -> Self {
                Self { x }
            }
        }
    }
)}
