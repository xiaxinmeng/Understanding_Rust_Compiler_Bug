rust
#[deny(non_exhaustive_omitted_patterns)]
match foo {
    Alphabet::A => {}
    Alphabet::B if guard => {}
    _ => {}
}
