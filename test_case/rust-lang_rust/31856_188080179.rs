 rust
macro_rules! take_mut {
    ($mut_ref:expr => $into:pat) => {
        let mut scope = Scope::new();
        let $into = Hole::new(&mut scope, $mut_ref);
    }
}
