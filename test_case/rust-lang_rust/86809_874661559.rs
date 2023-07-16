rust
match foo {
    Alphabet::A => {}
    // This has a guard, so the match isn't exhaustive, but both A and B are mentioned
    Alphabet::B if guard => {}
    #[deny(reachable)]
    _ => {}
}
