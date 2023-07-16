rust
// build with cargo test

mod tr {
    pub trait Trait: Sized { fn frob(self); }
    
    impl Trait for () { fn frob(self) {} }
}

#[cfg(test)]
mod c {
    use tr::Trait;
    mod c {
        use super::*;
        #[test] fn lol() { ().frob(); }
    }
}
