
    if let _ = (A{}) + A {
    } // `contains_exterior_struct_lit` says this should require parens!
    { }
