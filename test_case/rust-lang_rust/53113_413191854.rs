
struct Items<'a, X> where [X]: 'a {
    values: Cow<'a, [X]>,
}
