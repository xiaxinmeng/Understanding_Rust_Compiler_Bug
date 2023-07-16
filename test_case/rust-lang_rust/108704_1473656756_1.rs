rust
pub fn breaks(mut root: Vec<Node>) {
    let mut null = Vec::new();
    let mut elements = &mut root;

    {
        if let Some(el) = elements.get_mut(0) {
            elements = &mut el.children;
        } else {
            elements = &mut null;
        }
    }

    elements.clear();
}
