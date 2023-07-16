rust
existential type X<A, B>;

fn foo<A>(a: A) -> X<A, u32> {
    (a, 5u32)
}
