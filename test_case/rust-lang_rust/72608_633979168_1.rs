rust
macro_rules! gl_enums {
    ($mod:ident) => {
        not_serde::foo! {
            $mod::DEPTH_COMPONENT
        }
    }
}
gl_enums!(gl);
