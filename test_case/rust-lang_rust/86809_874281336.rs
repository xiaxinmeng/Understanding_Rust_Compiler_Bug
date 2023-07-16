rs
/// Latins are still figuring this out, please stand by
#[non_exhaustive]
enum Alphabet {
    A,
    B,
}

match foo {
    Alphabet::A => {}
    // This has a guard, so the match isn't exhaustive, but both A and B are mentioned
    Alphabet::B if guard => {}
    _ => {}
}
