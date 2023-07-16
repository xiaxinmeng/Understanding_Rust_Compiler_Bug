
fn merge_tree(root: &mut Root, path: Vec<String>) {
    let mut elements = &mut root.children;

    if let Some((idx, el)) = elements.iter_mut().enumerate().next() {
        // elements = &mut elements[idx].children;
        elements = &mut el.children;
    }
    if let Some((idx, el)) = elements.iter_mut().enumerate().next() {
        // elements = &mut elements[idx].children;
        elements = &mut el.children;
    }
}
