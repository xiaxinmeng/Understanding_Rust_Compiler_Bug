rust
// my use-case, a single value is most common
enum OccupiedSmallVec {
     Single(Foo),
     Multiple(Vec<Foo>),
}
