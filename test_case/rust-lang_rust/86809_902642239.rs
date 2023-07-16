rust
match (foo, bar) {
    (Foo::V, Bar::V) => {},
    #[warn(reachable)]
    _ => {},
}
