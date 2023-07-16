 rust
pub enum NodeData {
    Element(ElementData),
    Text(String),
    Comment(String),
    Doctype(Doctype),
    Document(DocumentData),
}

pub struct Node {
    // ...
    pub data: RefCell<NodeData>,
}

impl Node {
    fn as_element(&self) -> Option<Ref<ElementData>> {
        map_ref(self.data.borrow(), |data| match *data {
            Element(ref element) => Some(element),
            _ => None
        })
    }
}
