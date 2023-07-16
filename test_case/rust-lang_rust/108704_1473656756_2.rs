rust
pub fn breaks(mut root: Vec<Node>) {
    let mut elements = &mut root;

    {
        if let Some(el) = elements.deref_mut().get_mut(0) { // element's ownership is taken away by deref_mut
            elements = &mut el.children; // this assignment extends the duration of ownership taken by deref_mut to the same as the lifetime of element
        } else {
            // Do Nothing
        }
        // element was borrowed all the lifetime, but only one branch returned it
    }

    elements.clear();
}
