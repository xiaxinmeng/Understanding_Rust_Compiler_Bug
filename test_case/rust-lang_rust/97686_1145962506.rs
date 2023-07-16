rust
struct Tree<'a> {
    nested: std::collections::BTreeMap<&'a str, Tree<'a>>,
}

impl<'a> Tree<'a> {
    fn flatten(&self) -> impl Iterator<Item = &Tree<'a>> {
        Box::new(std::iter::once(self).chain(self.nested.values().flat_map(|tree| tree.flatten())))
            as Box<dyn Iterator<Item = &Tree<'a>>>
    }
}
