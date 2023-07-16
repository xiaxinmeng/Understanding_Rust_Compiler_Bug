rust
    let mut iter = (&b as &(dyn Error)).iter_chain();
    // or
    let b = Box::<Error>::from(b); let mut iter = b.iter_chain();
    // or
    let mut iter = <dyn Error>::iter_chain(&b);
    // or
    let mut iter = Error::iter_chain(&b);
    // but not
    let mut iter = b.iter_chain();
