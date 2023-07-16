 rust
pub struct ElementData {
    pub name: QualName,
    pub attributes: RefCell<HashMap<QualName, String>>,
}

impl ElementData {
    fn get_attribute(&self, name: Atom) -> Option<Ref<str>> {
        // With `map_ref` as proposed:
        map_ref(self.attributes.borrow(), |attrs| {
            attrs.get(&QualifiedName { ns: ns!(""), local: name }).map(|s| &**s)
        })

        // With a simplified `map_ref`, two HashMap lookups:
        let borrow = self.attributes.borrow();
        let key = QualifiedName { ns: ns!(""), local: name };
        if borrow.contains_key(&key) {
            Some(map_ref(borrow, |attrs| &*attrs[&key]))
        } else {
            None
        }
    }
}
