rust
macro outer {
    macro inner {
        #crate::Struct //  inner_crate::Struct
    }
}
