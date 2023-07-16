rust
struct FlatMap<Iter<'a>, FlatMap<Iter<'a>>, ...> {
    iter: Iter<'a>,
    subiter: Option<Self> // recursive
}
