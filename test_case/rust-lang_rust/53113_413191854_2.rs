
struct Items<'a, X> where [X]: 'a + ToOwned {
    values: Cow<'a, [X]>,
}
