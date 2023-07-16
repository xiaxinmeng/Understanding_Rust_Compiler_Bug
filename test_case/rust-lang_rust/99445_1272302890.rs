rust
macro outer {
    macro inner {
        crate::Struct // outer_crate::Struct
    }
}
