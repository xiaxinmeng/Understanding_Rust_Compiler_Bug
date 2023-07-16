 rust
impl Node {
    fn as_element(&self) -> Option<Ref<ElementData>> {
        let borrow = self.data.borrow();
        match *borrow {
            Element(_) => map_ref(borrow, |data| match *data {
                Element(ref element) => element,
                _ => unreachable!()
            }),
            _ => None
        }
    }
}
