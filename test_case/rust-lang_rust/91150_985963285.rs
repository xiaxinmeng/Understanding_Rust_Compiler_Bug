rust
macro_rules! m {
    (<$t:ty as $p:path>::$name:ident) => {
        type $name = <$t as $p>::$name;
    };
}

m!(<str as ToOwned>::Owned);
