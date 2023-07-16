rust
    let b1r: &Box<dyn Mine> = &b1;
    let b2r: &Box<dyn Mine> = &b2;
    if <Box<dyn Mine> as PartialEq>::eq(b1r, b2r) {}
    if <Box<dyn Mine> as PartialEq>::eq(b1r, b2r) {}
