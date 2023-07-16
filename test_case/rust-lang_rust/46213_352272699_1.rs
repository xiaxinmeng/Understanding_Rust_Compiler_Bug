rust
#![feature(untagged_unions)]
union OccupiedSmallVec {
    // where `single.0 == 0`
    single: (usize, Foo),
    multiple: Vec<Foo>,
}
