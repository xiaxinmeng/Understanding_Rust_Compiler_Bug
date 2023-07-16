rust
macro_rules! m {
    (& $($p:lifetime)? $i:ident) => {}
}

m!(&foo);
m!(&'a foo);
