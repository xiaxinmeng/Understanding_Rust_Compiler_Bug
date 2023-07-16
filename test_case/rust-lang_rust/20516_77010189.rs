 rust
#[lang="range_full"]
// Implicitly exported, but not in the usual way
macro_rules! name_doesnt_matter {
    () => ( $crate::ops::RangeFull )
}
