rust
#![feature(existential_type)]

existential type T: ;
fn test<A>() -> T {
    || {}
}
