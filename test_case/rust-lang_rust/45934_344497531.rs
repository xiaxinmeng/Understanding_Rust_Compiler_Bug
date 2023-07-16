rust
#![feature(decl_macro)]

macro m($i: ident) { // The token that substitutes `$i` resolves at the macro invocation
    mod foo {
        use $i::*;
    }
}

m!(super); // When resolved here, "super" doesn't make sense since the crate root has no parent.
