rust
pub fn new<L, R>(list: L) -> Counts<T>
    where L: IntoIterator<Item = R>,
            R: Borrow<T>,
            T: Clone,
{
    let mut fc = BTreeMap::new();
    let mut count = 0;
    for it in list {
        let entry = fc.entry(it.borrow().clone()).or_insert(0);
        *entry += 1;
        count += 1
    }

    Counts {
        counts: fc,
        total: count,
    }
}
