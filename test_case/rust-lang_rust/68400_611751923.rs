rust
struct A<const N: usize>;

impl A<7> {
    fn test<const M: bool>() {}
}

impl A<42> {
    fn test<const M: Option<()>>() {}
}

let a = A::<7>;
a.test::<true>();
