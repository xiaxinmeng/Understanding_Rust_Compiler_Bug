 rust
macro_rules! render_function {
    ($name:ident) => (
        impl S {
            pub fn $name(output: &mut i32, input: &i32) -> i32 {
                5i32
            }
        }
    );
}
