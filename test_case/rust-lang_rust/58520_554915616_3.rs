rust
    let mut iter = (&b as &(dyn Error)).chain();
    // or
    let mut iter = <dyn Error>::chain(&b);
    // or
    let mut iter = Error::chain(&b);
