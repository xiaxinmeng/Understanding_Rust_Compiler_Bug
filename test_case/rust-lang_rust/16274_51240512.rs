 rust
#![feature(globs)]

pub mod longhands {
    pub use super::*;

    pub use to_computed_value = super::common_types::computed::compute_CSSColor;

    pub fn computed_as_specified() {}
}

pub mod common_types {
    pub mod computed {
        pub use compute_CSSColor = super::super::longhands::computed_as_specified;
    }
}
