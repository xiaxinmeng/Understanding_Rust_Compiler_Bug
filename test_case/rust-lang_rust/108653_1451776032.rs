rs
enum Enum {
    IDENT,
}

/// [`Self::IDENT`]
impl Trait for Enum {
    type IDENT = usize;

    const IDENT: usize = 10;
}
